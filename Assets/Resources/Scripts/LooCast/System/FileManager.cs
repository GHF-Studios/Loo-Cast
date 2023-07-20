using System;
using System.Collections.Generic;
using System.Linq;

namespace LooCast.System
{
    using LooCast.System.Paths;

    public sealed class FileManager : ModuleManager
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
        private FileManager() : base("FileManager", SystemManager.Instance)
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

        public bool TryGetFile(FilePath filePath, out IFile file)
        {
            if (!registeredFiles.ContainsKey(filePath))
            {
                file = null;
                return false;
            }
            else
            {
                file = registeredFiles[filePath];
                return true;
            }
        }

        public IFile GetFile(string fileGUSP)
        {
            if (!FilePath.TryParse(fileGUSP, out FilePath? filePath))
            {
                return null;
            }
            return GetFile(filePath!);
        }

        public bool TryGetFile(string stringFilePath, out IFile file)
        {
            if (!FilePath.TryParse(stringFilePath, out FilePath? filePath))
            {
                file = null;
                return false;
            }
            return TryGetFile(filePath!, out file);
        }

        public bool FileExists(FilePath filePath)
        {
            return registeredFiles.ContainsKey(filePath);
        }
        #endregion
    }
}
