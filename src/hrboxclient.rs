use std::fs::File;
use std::sync::Arc;
use std::{error::Error, path::Path};

use log::{info, trace};
use reqwest::header::{self, HeaderMap};
use reqwest::{blocking::Client, cookie::Jar, redirect::Policy};

use crate::hrdocumentbox::{Document, HrDocumentBox};

pub struct HrBoxClient {
    base_api_url: String,
    client: Client,
}

impl HrBoxClient {
    // ---------- public functions ----------

    /// Downloads a file from the HR box.
    /// NOTE(MSt): Currently this only works with PDFs.
    /// It might need different URLs once different file types are needed.
    /// Also the content-disposition header is ignored.
    pub fn download_file(
        &self,
        document: &Document,
        output_folder: &Path,
    ) -> Result<(), Box<dyn Error>> {
        let pdf_url = format!(
            "{}/internal/documents/{}/pdf",
            self.base_api_url, document.file_index
        );

        trace!(
            "Downloading [{:?}] from url [{:?}].",
            document.name,
            pdf_url
        );

        let mut output_folder = output_folder.to_path_buf();
        output_folder.push(format!("{}.pdf", document.name));

        info!("Saving pdf to [{:?}].", output_folder);

        let mut pdf_response = self.client.get(pdf_url).send()?.error_for_status()?;

        let mut file = File::create(output_folder)?;
        pdf_response.copy_to(&mut file)?;

        Ok(())
    }

    /// Retrieves every document from the HR box. This includes multiple
    /// requests because the results are paginated.
    pub fn get_all_documents(&self) -> Result<HrDocumentBox, Box<dyn Error>> {
        let mut document_box = self.get_documents(0)?;

        info!(
            "Retrieved information for [{:?}/{:?}] documents.",
            document_box.total_result_count, document_box.total_count
        );

        let mut count = document_box.total_result_count;
        while count < document_box.total_count {
            let mut temp_document_box = self.get_documents(count)?;
            document_box
                .documents
                .append(&mut temp_document_box.documents);

            count += temp_document_box.total_result_count;

            info!(
                "Retrieved information for [{:?}/{:?}] documents.",
                count, document_box.total_count
            );
        }

        info!(
            "Retrieved information for a total of [{:?}] documents.",
            count
        );

        Ok(document_box)
    }

    /// Authenticates to the HR box using the given username and password.
    pub fn login(&self, username: &str, password: &str) -> Result<(), Box<dyn Error>> {
        let login_url = format!("{}/external/login", self.base_api_url);

        trace!("Log-in using username=[{:?}], password=[*****]", username);

        // Do the actual login by posting the login data.
        let params = [("username", username), ("password", password)];
        self.client
            .post(login_url)
            .form(&params)
            .send()?
            .error_for_status()?;

        Ok(())
    }

    /// Creates a new instance of the HrBoxClient.
    /// This will already call the HR box API using get_xsrf_token().
    pub fn new(subdomain: &str) -> Result<Self, Box<dyn Error>> {
        let base_url = format!("https://{subdomain}.hr-document-box.com");
        info!("Base URL [{:?}].", base_url);

        let cookie_jar = Arc::new(Jar::default());
        let xsrf_token = HrBoxClient::get_xsrf_token(&base_url, cookie_jar.clone())?;

        let mut headers = HeaderMap::new();
        headers.append("X-XSRF-TOKEN", header::HeaderValue::from_str(&xsrf_token)?);
        trace!("Default headers: [{:?}].", headers);

        Ok(HrBoxClient {
            base_api_url: format!("{base_url}/api/v1/"),
            client: Client::builder()
                .cookie_provider(cookie_jar)
                .default_headers(headers)
                .build()?,
        })
    }

    // ---------- private functions ----------

    /// Gets the given init URL to retrieve the cookies and XSRF token that are
    /// needed for a successful login.
    fn get_xsrf_token(
        login_init_url: &str,
        cookie_jar: Arc<Jar>,
    ) -> Result<String, Box<dyn Error>> {
        let client = Client::builder()
            .cookie_provider(cookie_jar)
            .redirect(Policy::none())
            .build()?;

        // Get the initial web page to retrieve the XSRF token in form of a cookie
        let response = client.get(login_init_url).send()?.error_for_status()?;

        for cookie in response.cookies() {
            if cookie.name() == "XSRF-TOKEN" {
                trace!("XSRF token [{:?}].", cookie.value());
                return Ok(cookie.value().to_string());
            }
        }

        Err("No \"XSRF-TOKEN\" cookie found.".into())
    }

    /// Gets one page of the HR box documents, starting with the given offset.
    fn get_documents(&self, offset: u32) -> Result<HrDocumentBox, Box<dyn Error>> {
        let document_url = format!("{}internal/documents?offset={}", self.base_api_url, offset);
        let document_box = self
            .client
            .get(document_url)
            .send()?
            .json::<HrDocumentBox>()?;

        Ok(document_box)
    }
}
