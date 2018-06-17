use sgx_trts::*;
use sgx_types::*;
use sgx_tse::*;

use std::string::String;
use std::vec::Vec;
use std::io::{self, Write};
use std::slice;

// extra_data size (limit to 64)
static REPORT_DATA_SIZE : usize = 64;

pub fn create_report_with_data(target_info: &sgx_target_info_t , out_report: &mut sgx_report_t , extra_data : &[u8] ) -> sgx_status_t {
    let reportDataSize : usize = 64;
    let mut report_data = sgx_report_data_t::default();

    // secret data to be attached with the report.
    if extra_data.len() > REPORT_DATA_SIZE {
        return sgx_status_t::SGX_ERROR_INVALID_PARAMETER
    }

    report_data.d[..extra_data.len()].copy_from_slice(extra_data);

    let mut report = match rsgx_create_report(&target_info, &report_data) {
        Ok(r) =>{
           *out_report = r;
            sgx_status_t::SGX_SUCCESS
        },
        Err(r) =>{
            println!("Report creationg => failed" );
            r
        },
    };
    sgx_status_t::SGX_SUCCESS
}