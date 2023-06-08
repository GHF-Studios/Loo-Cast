namespace LooCast.System.Paths
{
    public static class PathUtil
    {
        public static bool IsValidSubPath(string subPath)
        {
            return !string.IsNullOrEmpty(subPath) && !string.IsNullOrWhiteSpace(subPath) && StringUtil.IsAlphaNumeric(subPath);
        }

        public static bool IsValidPath(string path)
        {
            return !StringUtil.IsEmpty(path) && StringUtil.IsAlphaNumericWithExceptions(path, '/', '.', ':', '+');
        }
        
        public static bool IsValidFolderPath(string folderPath)
        {
            return !StringUtil.IsEmpty(folderPath) && StringUtil.IsAlphaNumericWithExceptions(folderPath, '/');
        }
        
        public static bool IsValidFilePath(string filePath)
        {
            return !StringUtil.IsEmpty(filePath) && StringUtil.IsAlphaNumericWithExceptions(filePath, '/', '.');
        }

        public static bool IsValidObjectPath(string objectPath)
        {
            return !StringUtil.IsEmpty(objectPath) && StringUtil.IsAlphaNumericWithExceptions(objectPath, '/', '.', ':', '+');
        }
    }
}
