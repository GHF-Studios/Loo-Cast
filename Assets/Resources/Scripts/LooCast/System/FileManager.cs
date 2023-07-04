using System.Collections.Generic;

namespace LooCast.System
{
    using global::System;
    using LooCast.System.Paths;

    public class FileManager : ModuleManager
    {
        #region Static Properties
        public static FileManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new FileManager();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static FileManager instance;
        #endregion

        #region Properties
        #endregion

        #region Fields
        private Dictionary<FilePath, IFile> registeredFiles;
        #endregion

        #region Constructors
        public FileManager() : base("FileManager", SystemManager.Instance)
        {
            registeredFiles = new Dictionary<FilePath, IFile>();
        }
        #endregion

        #region Methods
        public void RegisterFile(IFile file)
        {
            if (!registeredFiles.ContainsKey(file.FilePath))
            {
                registeredFiles.Add(file.FilePath, file);
            }
        }

        public void UnregisterFile(IFile file)
        {
            if (registeredFiles.ContainsKey(file.FilePath))
            {
                registeredFiles.Remove(file.FilePath);
            }
        }

        public IFile GetFile(FilePath filePath)
        {
            if (registeredFiles.ContainsKey(filePath))
            {
                return registeredFiles[filePath];
            }
            return null;
        }

        public IFile GetFile(string stringFilePath)
        {
            if (!FilePath.TryParse(stringFilePath, out FilePath? filePath))
            {
                return null;
            }
            return GetFile(filePath!);
        }

        public bool FileExists(FilePath filePath)
        {
            return registeredFiles.ContainsKey(filePath);
        }

        public void CreateFile(FilePath filePath)
        {
            
        }
        #endregion
    }
}
