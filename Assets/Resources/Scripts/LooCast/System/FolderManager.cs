using System.Collections.Generic;

namespace LooCast.System
{
    using global::System;
    using LooCast.System.Paths;

    public class FolderManager : ModuleManager
    {
        #region Static Properties
        public static FolderManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new FolderManager();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static FolderManager instance;
        #endregion

        #region Properties
        #endregion

        #region Fields
        private Dictionary<FolderPath, IFolder> registeredFolders;
        #endregion

        #region Constructors
        public FolderManager() : base("FolderManager", SystemManager.Instance)
        {
            registeredFolders = new Dictionary<FolderPath, IFolder>();
        }
        #endregion

        #region Methods
        public void RegisterFolder(IFolder folder)
        {
            if (!registeredFolders.ContainsKey(folder.FolderPath))
            {
                registeredFolders.Add(folder.FolderPath, folder);
            }
        }

        public void UnregisterFolder(IFolder folder)
        {
            if (registeredFolders.ContainsKey(folder.FolderPath))
            {
                registeredFolders.Remove(folder.FolderPath);
            }
        }

        public IFolder GetFolder(FolderPath folderPath)
        {
            if (registeredFolders.ContainsKey(folderPath))
            {
                return registeredFolders[folderPath];
            }
            return null;
        }

        public IFolder GetFolder(string stringFolderPath)
        {
            if (!FolderPath.TryParse(stringFolderPath, out FolderPath? folderPath))
            {
                return null;
            }
            return GetFolder(folderPath!);
        }

        public bool FolderExists(FolderPath folderPath)
        {
            return registeredFolders.ContainsKey(folderPath);
        }

        public void CreateFolder(FolderPath folderPath)
        {
            if (folderPath == null)
            {
                throw new ArgumentNullException(nameof(folderPath));
            }

            if (FolderExists(folderPath))
            {
                return;
            }

            FolderPath parentFolderPath = (FolderPath)folderPath.ParentFolderPath;
            IFolder parentFolder = GetFolder(parentFolderPath);

            if (parentFolder == null)
            {
                CreateFolder(parentFolderPath);
                parentFolder = GetFolder(parentFolderPath);
            }

            IFolder folder = new Folder(folderPath.FolderName, parentFolder);
            RegisterFolder(folder);
        }
        #endregion
    }
}
