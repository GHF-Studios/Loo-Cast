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
                return fileFullName == null && objectNames.Count == 0;
            }
        }
        private bool isFilePath
        {
            get
            {
                return fileFullName != null && objectNames.Count == 0;
            }
        }
        private bool isObjectPath
        {
            get
            {
                return fileFullName != null && objectNames.Count != 0;
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

        private PathBuilder(FolderPath folderPath)
        {
            folderNames = new List<string>(folderPath.FolderNames);
            fileName = null;
            fileExtension = null;
            objectNames = new List<string>();
            isRelative = folderPath.IsRelative;
        }

        private PathBuilder(FilePath filePath)
        {
            folderNames = new List<string>(filePath.FolderPathParent.FolderNames);
            fileName = filePath.FileName;
            fileExtension = filePath.FileExtension;
            objectNames = new List<string>();
            isRelative = filePath.IsRelative;
        }

        private PathBuilder(ObjectPath objectPath)
        {
            folderNames = new List<string>(objectPath.FilePathParent.FolderPathParent.FolderNames);
            fileName = objectPath.FilePathParent.FileName;
            fileExtension = objectPath.FilePathParent.FileExtension;
            objectNames = new List<string>(objectPath.ObjectNames);
            isRelative = objectPath.IsRelative;
        }
        #endregion

        #region Static Methods
        public static PathBuilder Create()
        {
            return new PathBuilder();
        }

        public static PathBuilder Load(FolderPath folderPath)
        {
            return new PathBuilder(folderPath);
        }

        public static PathBuilder Load(FilePath filePath)
        {
            return new PathBuilder(filePath);
        }

        public static PathBuilder Load(ObjectPath objectPath)
        {
            return new PathBuilder(objectPath);
        }
        #endregion

        #region Methods
        public void AsRelativePath()
        {
            isRelative = true;
        }

        public void AsAbsolutePath()
        {
            isRelative = false;
        }

        public void WithFolder(string folderName)
        {
            if (isRelative == null)
            {
                throw new InvalidOperationException("Must specify whether path is relative or absolute before construction!");
            }
            if (!isFolderPath || isFilePath || isObjectPath)
            {
                throw new InvalidOperationException("A folder can only be contained within another folder!");
            }

            folderNames.Add(folderName);
        }
        public void WithFolder(FolderPath folderPath)
        {
            if (isRelative == null)
            {
                throw new InvalidOperationException("Must specify whether path is relative or absolute before construction!");
            }
            if (!isFolderPath || isFilePath || isObjectPath)
            {
                throw new InvalidOperationException("A folder can only be contained within another folder!");
            }

            folderNames.AddRange(folderPath.FolderNames);
        }

        public void WithFile(string fileName, string fileExtension)
        {
            if (isRelative == null)
            {
                throw new InvalidOperationException("Must specify whether path is relative or absolute before construction!");
            }
            if (!isFolderPath || isFilePath || isObjectPath)
            {
                throw new InvalidOperationException("A file can only be contained within a folder!");
            }

            this.fileName = fileName;
            this.fileExtension = fileExtension;
        }
        public void WithFile(FilePath filePath)
        {
            if (isRelative == null)
            {
                throw new InvalidOperationException("Must specify whether path is relative or absolute before construction!");
            }
            if (!isFolderPath || isFilePath || isObjectPath)
            {
                throw new InvalidOperationException("A file can only be contained within a folder!");
            }

            fileName = filePath.FileName;
            fileExtension = filePath.FileExtension;
        }

        public void WithObject(string objectName)
        {
            if (isRelative == null)
            {
                throw new InvalidOperationException("Must specify whether path is relative or absolute before construction!");
            }
            if (isFolderPath || (!isFilePath && !isObjectPath))
            {
                throw new InvalidOperationException("An object can only be contained within another object or a file!");
            }

            objectNames.Add(objectName);
        }
        public void WithObject(ObjectPath objectPath)
        {
            if (isRelative == null)
            {
                throw new InvalidOperationException("Must specify whether path is relative or absolute before construction!");
            }
            if (isFolderPath || (!isFilePath && !isObjectPath))
            {
                throw new InvalidOperationException("An object can only be contained within another object or a file!");
            }

            objectNames.AddRange(objectPath.ObjectNames);
        }

        public FolderPath ConstructFolderPath()
        {
            return new FolderPath((bool)isRelative, folderNames.ToArray());
        }

        public FilePath ConstructFilePath()
        {
            return new FilePath((bool)isRelative, fileName, fileExtension, ConstructFolderPath()); 
        }

        public ObjectPath ConstructObjectPath()
        {
            return new ObjectPath((bool)isRelative, ConstructFilePath(), objectNames.ToArray());
        }
        #endregion
    }
}
