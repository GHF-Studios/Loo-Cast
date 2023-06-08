using System;
using System.Collections.Generic;

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
            
        }
        
        public static bool IsValidFolderPath(string folderPath)
        {
        }
        
        public static bool IsValidFilePath(string filePath)
        {
        }

        public static bool IsValidObjectPath(string objectPath)
        {
        }
    }
}
