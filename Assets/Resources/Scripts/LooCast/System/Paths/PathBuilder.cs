using System;
using System.Collections.Generic;

namespace LooCast.System.Paths
{
    public class PathBuilder
    {
        #region Fields
        private List<string> folderNames;
        private string fileFullName
        {
            get
            {
                if (!StringUtil.IsAlphaNumeric(fileName) || !StringUtil.IsAlphaNumeric(fileExtension))
                {
                    return null;
                }
                return $"{fileName}.{fileExtension}";
            }
        }
        private string fileName;
        private string fileExtension;
        private List<string> objectNames;
        private bool? isRelative;
        private bool isFolderPath
        {
            get
            {
                return folderNames.Count != 0 && fileFullName == null && objectNames.Count == 0;
            }
        }
        private bool isFilePath
        {
            get
            {
                return (folderNames.Count != 0 && fileFullName != null && objectNames.Count == 0) || (folderNames.Count == 0 && fileFullName != null && objectNames.Count == 0);
            }
        }
        private bool isObjectPath
        {
            get
            {
                return (folderNames.Count != 0 && fileFullName != null && objectNames.Count != 0) || (folderNames.Count == 0 && fileFullName != null && objectNames.Count != 0) || (folderNames.Count == 0 && fileFullName == null && objectNames.Count != 0);
            }
        }
        #endregion

        #region Constructors
        private PathBuilder()
        {
            folderNames = new List<string>();
            fileName = null;
            fileExtension = null;
            objectNames = new List<string>();
            isRelative = null;
        }

        private PathBuilder(string gusp)
        {
            folderNames = new List<string>();
            fileName = null;
            fileExtension = null;
            objectNames = new List<string>();
            isRelative = null;
        }
        #endregion

        #region Static Methods
        public static PathBuilder Create()
        {
            return new PathBuilder();
        }

        public static PathBuilder Load(string gusp)
        {
            return new PathBuilder(gusp);
        }
        #endregion

        public PathBuilder AsRelativePath()
        {
            isRelative = true;
            return this;
        }

        public PathBuilder AsAbsolutePath()
        {
            isRelative = false;
            return this;
        }

        public PathBuilder WithFolder(string folderName)
        {
            if (!isFolderPath || isFilePath || isObjectPath)
            {
                throw new InvalidOperationException("A folder can only be contained within another folder!");
            }

            folderNames.Add(folderName);
            return this;
        }

        public PathBuilder WithFile(string fileName, string fileExtension)
        {
            if (!isFolderPath || isFilePath || isObjectPath)
            {
                throw new InvalidOperationException("A file can only be contained within a folder!");
            }

            this.fileName = fileName;
            this.fileExtension = fileExtension;
            return this;
        }
            
        public PathBuilder WithObject(string objectName)
        {
            if (isFolderPath || (!isFilePath && !isObjectPath))
            {
                throw new InvalidOperationException("An object can only be contained within another object or a file!");
            }

            objectNames.Add(objectName);
            return this;
        }

        public FolderPath ConstructFolderPath()
        {
            if (isRelative == null)
            {
                throw new InvalidOperationException("Must specify whether path is relative or absolute before construction!");
            }
        }

        public FilePath ConstructFilePath()
        {
            if (isRelative == null)
            {
                throw new InvalidOperationException("Must specify whether path is relative or absolute before construction!");
            }

            string gusp = "";

            if (!(bool)isRelative)
            {
                gusp += "/";
            }
        }

        public ObjectPath ConstructObjectPath()
        {
            if (isRelative == null)
            {
                throw new InvalidOperationException("Must specify whether path is relative or absolute before construction!");
            }

            string gusp = "";

            if (!(bool)isRelative)
            {
                gusp += "/";
            }
        }
    }
}
