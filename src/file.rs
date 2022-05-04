use windows::{
    core::{Error, Result, PWSTR},
    Storage::{CreationCollisionOption, StorageFile, StorageFolder},
    Win32::Storage::FileSystem::GetFullPathNameW,
};

pub fn get_full_path_name(input_path: &str) -> Result<String> {
    let mut buffer = Vec::<u16>::new();
    let len = unsafe { GetFullPathNameW(input_path, &mut buffer, std::ptr::null_mut()) };
    if len == 0 {
        return Err(Error::from_win32());
    }
    buffer.resize(len as usize, 0);
    let len = unsafe { GetFullPathNameW(input_path, &mut buffer, std::ptr::null_mut()) };
    if len == 0 {
        return Err(Error::from_win32());
    }
    buffer.resize(len as usize, 0);
    let full_path = String::from_utf16(&buffer).unwrap();
    Ok(full_path)
}

pub fn get_parent_folder_path_and_file_name(input_path: &str) -> Result<(String, String)> {
    let mut buffer = Vec::<u16>::new();
    let len = unsafe { GetFullPathNameW(input_path, &mut buffer, std::ptr::null_mut()) };
    if len == 0 {
        return Err(Error::from_win32());
    }
    buffer.resize(len as usize, 0);
    let mut file_part = PWSTR(std::ptr::null_mut());
    let len = unsafe { GetFullPathNameW(input_path, &mut buffer, &mut file_part) };
    if len == 0 {
        return Err(Error::from_win32());
    }
    buffer.resize(len as usize, 0);

    let buffer_ptr = buffer.as_ptr();
    let folder_path_len = (file_part.0 as usize - buffer_ptr as usize) / std::mem::size_of::<u16>();

    let parent_folder_path = String::from_utf16(&buffer[..folder_path_len]).unwrap();
    let file_name = String::from_utf16(&buffer[folder_path_len..]).unwrap();

    Ok((parent_folder_path, file_name))
}

pub fn create_storage_file_from_path(
    path: &str,
    options: CreationCollisionOption,
) -> Result<StorageFile> {
    let (parent_folder, file_name) = get_parent_folder_path_and_file_name(path)?;
    let storage_folder = StorageFolder::GetFolderFromPathAsync(parent_folder)?.get()?;
    let storage_file = storage_folder.CreateFileAsync(file_name, options)?.get()?;
    Ok(storage_file)
}
