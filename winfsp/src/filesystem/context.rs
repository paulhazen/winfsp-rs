use crate::error::Result;
use crate::filesystem::{DirInfo, DirMarker, FileInfo, OpenFileInfo, VolumeInfo};
use crate::U16CStr;

use windows::Win32::Foundation::STATUS_INVALID_DEVICE_REQUEST;
use windows::Win32::Security::PSECURITY_DESCRIPTOR;
use windows::Win32::Storage::FileSystem::{FILE_ACCESS_FLAGS, FILE_FLAGS_AND_ATTRIBUTES};

use winfsp_sys::{FSP_FSCTL_TRANSACT_REQ, FSP_FSCTL_TRANSACT_RSP};

#[derive(Debug)]
/// The return value of a request to [`FileSystemContext::get_security_by_name`](crate::filesystem::FileSystemContext::get_security_by_name).
pub struct FileSecurity {
    /// When a file name containing reparse points anywhere but the final path component is encountered,
    /// this should be true.
    pub reparse: bool,

    /// The size of the security descriptor needed to hold security information about the file.
    pub sz_security_descriptor: u64,

    /// The file attributes of the file.
    pub attributes: u32,
}

#[derive(Debug)]
/// The return value of a request to [`FileSystemContext::read`](crate::filesystem::FileSystemContext::read) or
/// [`FileSystemContext::write`](crate::filesystem::FileSystemContext::write).
pub struct IoResult {
    /// The number of bytes transferred in the IO request.
    pub bytes_transferred: u32,

    /// If the operation is asynchronous, whether or not the request is pending.
    pub io_pending: bool,
}

#[allow(unused_variables)]
/// The core trait that implements file system operations for a WinFSP file system.
///
/// If an implementor of this trait panics in any of the methods,
/// the caller will receive `STATUS_NONCONTINUABLE_EXCEPTION` (0xC0000025).
///
/// Any non-implemented optional methods will return `STATUS_INVALID_DEVICE_REQUEST` (0xC0000010).
pub trait FileSystemContext: Sized {
    /// The user context that represents an open handle in the file system.
    ///
    /// The semantics of `FileContext` vary depending on the volume parameters
    /// used to mount the file system. See [`FileContextMode`](crate::host::FileContextMode)
    /// for more information.
    type FileContext: Sized;

    /// Get security information and attributes for a file or directory by its file name.
    ///
    /// If the file system supports reparse points, `reparse_point_resolver` should be
    /// called with the input file_name. If a reparse point is found at any point
    /// in the path, the result can be immediately returned like so the following.
    ///
    /// ```
    /// if let Some(security) = resolve_reparse_points(file_name.as_ref()) {
    ///    Ok(security)
    /// }
    /// ```
    fn get_security_by_name<P: AsRef<U16CStr>>(
        &self,
        file_name: P,
        security_descriptor: PSECURITY_DESCRIPTOR,
        descriptor_len: Option<u64>,
        reparse_point_resolver: impl FnOnce(&U16CStr) -> Option<FileSecurity>,
    ) -> Result<FileSecurity>;

    /// Opens a file or a directory.
    fn open<P: AsRef<U16CStr>>(
        &self,
        file_name: P,
        create_options: u32,
        granted_access: FILE_ACCESS_FLAGS,
        file_info: &mut OpenFileInfo,
    ) -> Result<Self::FileContext>;

    /// Close a file or directory handle.
    fn close(&self, context: Self::FileContext);

    #[allow(clippy::too_many_arguments)]
    /// Create a new file or directory.
    fn create<P: AsRef<U16CStr>>(
        &self,
        file_name: P,
        create_options: u32,
        granted_access: FILE_ACCESS_FLAGS,
        file_attributes: FILE_FLAGS_AND_ATTRIBUTES,
        security_descriptor: PSECURITY_DESCRIPTOR,
        allocation_size: u64,
        extra_buffer: Option<&[u8]>,
        extra_buffer_is_reparse_point: bool,
        file_info: &mut OpenFileInfo,
    ) -> Result<Self::FileContext> {
        Err(STATUS_INVALID_DEVICE_REQUEST.into())
    }

    /// Clean up a file.
    fn cleanup<P: AsRef<U16CStr>>(
        &self,
        context: &mut Self::FileContext,
        file_name: Option<P>,
        flags: u32,
    ) {
    }

    /// Flush a file or volume.
    ///
    /// If `context` is `None`, the request is to flush the entire volume.
    fn flush(&self, context: Option<&Self::FileContext>, file_info: &mut FileInfo) -> Result<()> {
        Err(STATUS_INVALID_DEVICE_REQUEST.into())
    }

    /// Get file or directory information.
    fn get_file_info(&self, context: &Self::FileContext, file_info: &mut FileInfo) -> Result<()> {
        Err(STATUS_INVALID_DEVICE_REQUEST.into())
    }

    /// Get file or directory security descriptor.
    fn get_security(
        &self,
        context: &Self::FileContext,
        security_descriptor: PSECURITY_DESCRIPTOR,
        descriptor_len: Option<u64>,
    ) -> Result<u64> {
        Err(STATUS_INVALID_DEVICE_REQUEST.into())
    }

    /// Set file or directory security descriptor.
    fn set_security(
        &self,
        context: &Self::FileContext,
        security_information: u32,
        modification_descriptor: PSECURITY_DESCRIPTOR,
    ) -> Result<()> {
        Err(STATUS_INVALID_DEVICE_REQUEST.into())
    }

    /// Overwrite a file.
    fn overwrite(
        &self,
        context: &Self::FileContext,
        file_attributes: FILE_FLAGS_AND_ATTRIBUTES,
        replace_file_attributes: bool,
        allocation_size: u64,
        extra_buffer: Option<&[u8]>,
        file_info: &mut FileInfo,
    ) -> Result<()> {
        Err(STATUS_INVALID_DEVICE_REQUEST.into())
    }

    /// Read directory entries from a directory handle.
    fn read_directory<P: AsRef<U16CStr>>(
        &self,
        context: &mut Self::FileContext,
        pattern: Option<P>,
        marker: DirMarker,
        buffer: &mut [u8],
    ) -> Result<u32> {
        Err(STATUS_INVALID_DEVICE_REQUEST.into())
    }

    /// Renames a file or directory.
    fn rename<P: AsRef<U16CStr>>(
        &self,
        context: &Self::FileContext,
        file_name: P,
        new_file_name: P,
        replace_if_exists: bool,
    ) -> Result<()> {
        Err(STATUS_INVALID_DEVICE_REQUEST.into())
    }

    /// Set file or directory basic information.
    #[allow(clippy::too_many_arguments)]
    fn set_basic_info(
        &self,
        context: &Self::FileContext,
        file_attributes: u32,
        creation_time: u64,
        last_access_time: u64,
        last_write_time: u64,
        last_change_time: u64,
        file_info: &mut FileInfo,
    ) -> Result<()> {
        Err(STATUS_INVALID_DEVICE_REQUEST.into())
    }

    /// Set the file delete flag.
    ///
    /// ## Safety
    /// The file should **never** be deleted in this function. Instead,
    /// set a flag to indicate that the file is to be deleted later by
    /// [`FileSystemContext::cleanup`](crate::filesystem::FileSystemContext::cleanup).
    fn set_delete<P: AsRef<U16CStr>>(
        &self,
        context: &Self::FileContext,
        file_name: P,
        delete_file: bool,
    ) -> Result<()> {
        Err(STATUS_INVALID_DEVICE_REQUEST.into())
    }

    /// Set the file or allocation size.
    fn set_file_size(
        &self,
        context: &Self::FileContext,
        new_size: u64,
        set_allocation_size: bool,
        file_info: &mut FileInfo,
    ) -> Result<()> {
        Err(STATUS_INVALID_DEVICE_REQUEST.into())
    }

    /// Read from a file.
    fn read(
        &self,
        context: &Self::FileContext,
        buffer: &mut [u8],
        offset: u64,
    ) -> Result<IoResult> {
        Err(STATUS_INVALID_DEVICE_REQUEST.into())
    }

    /// Write to a file.
    fn write(
        &self,
        context: &Self::FileContext,
        buffer: &[u8],
        offset: u64,
        write_to_eof: bool,
        constrained_io: bool,
        file_info: &mut FileInfo,
    ) -> Result<IoResult> {
        Err(STATUS_INVALID_DEVICE_REQUEST.into())
    }

    /// Get directory information for a single file or directory within a parent directory.
    ///
    /// This method is only called when [VolumeParams::pass_query_directory_filename](crate::host::VolumeParams::pass_query_directory_filename)
    /// is set to true, and the file system was created with [FileSystemParams::use_dir_info_by_name](crate::host::FileSystemParams).
    /// set to true.
    fn get_dir_info_by_name<P: AsRef<U16CStr>>(
        &self,
        context: &Self::FileContext,
        file_name: P,
        out_dir_info: &mut DirInfo,
    ) -> Result<()> {
        Err(STATUS_INVALID_DEVICE_REQUEST.into())
    }

    /// Get information about the volume.
    fn get_volume_info(&self, out_volume_info: &mut VolumeInfo) -> Result<()> {
        Err(STATUS_INVALID_DEVICE_REQUEST.into())
    }

    /// Set the volume label.
    fn set_volume_label<P: AsRef<U16CStr>>(
        &self,
        volume_label: P,
        volume_info: &mut VolumeInfo,
    ) -> Result<()> {
        Err(STATUS_INVALID_DEVICE_REQUEST.into())
    }

    /// Get information about named streams.
    fn get_stream_info(&self, context: &Self::FileContext, buffer: &mut [u8]) -> Result<u32> {
        Err(STATUS_INVALID_DEVICE_REQUEST.into())
    }

    /// Get reparse point information by its name.
    ///
    /// In the WinFSP C API, this method is usually called manually by the interface method
    /// `ResolveReparsePoints`. winfsp-rs automatically handles resolution of reparse points
    /// if this method is implemented properly.
    fn get_reparse_point_by_name<P: AsRef<U16CStr>>(
        &self,
        file_name: P,
        is_directory: bool,
        buffer: &mut [u8],
    ) -> Result<u64> {
        Err(STATUS_INVALID_DEVICE_REQUEST.into())
    }

    /// Get reparse point information.
    fn get_reparse_point<P: AsRef<U16CStr>>(
        &self,
        context: &Self::FileContext,
        file_name: P,
        buffer: &mut [u8],
    ) -> Result<u64> {
        Err(STATUS_INVALID_DEVICE_REQUEST.into())
    }

    /// Set reparse point information.
    fn set_reparse_point<P: AsRef<U16CStr>>(
        &self,
        context: &Self::FileContext,
        file_name: P,
        buffer: &[u8],
    ) -> Result<()> {
        Err(STATUS_INVALID_DEVICE_REQUEST.into())
    }

    /// Delete reparse point information.
    fn delete_reparse_point<P: AsRef<U16CStr>>(
        &self,
        context: &Self::FileContext,
        file_name: P,
        buffer: &[u8],
    ) -> Result<()> {
        Err(STATUS_INVALID_DEVICE_REQUEST.into())
    }

    /// Get extended attribute information.
    fn get_extended_attributes(
        &self,
        context: &Self::FileContext,
        buffer: &mut [u8],
    ) -> Result<u32> {
        Err(STATUS_INVALID_DEVICE_REQUEST.into())
    }

    /// Set extended attribute information.
    fn set_extended_attributes(
        &self,
        context: &Self::FileContext,
        buffer: &[u8],
        file_info: &mut FileInfo,
    ) -> Result<()> {
        Err(STATUS_INVALID_DEVICE_REQUEST.into())
    }

    /// Process a control code from the DeviceIoControl API.
    fn control(
        &self,
        context: &Self::FileContext,
        control_code: u32,
        input: &[u8],
        output: &mut [u8],
    ) -> Result<u32> {
        Err(STATUS_INVALID_DEVICE_REQUEST.into())
    }

    /// Get the context response of the current FSP interface operation.
    ///
    /// ## Safety
    /// This function may be used only when servicing one of the `FileSystemContext` operations.
    /// The current operation context is stored in thread local storage.
    unsafe fn with_operation_response<T, F>(&self, f: F) -> Option<T>
    where
        F: FnOnce(&mut FSP_FSCTL_TRANSACT_RSP) -> T,
    {
        unsafe {
            if let Some(context) = winfsp_sys::FspFileSystemGetOperationContext().as_ref() {
                if let Some(response) = context.Response.as_mut() {
                    return Some(f(response));
                }
            }
        }
        None
    }

    /// Get the context request of the current FSP interface operation.
    ///
    /// ## Safety
    /// This function may be used only when servicing one of the `FileSystemContext` operations.
    /// The current operation context is stored in thread local storage.
    unsafe fn with_operation_request<T, F>(&self, f: F) -> Option<T>
    where
        F: FnOnce(&FSP_FSCTL_TRANSACT_REQ) -> T,
    {
        unsafe {
            if let Some(context) = winfsp_sys::FspFileSystemGetOperationContext().as_ref() {
                if let Some(request) = context.Request.as_ref() {
                    return Some(f(request));
                }
            }
        }
        None
    }
}
