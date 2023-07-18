﻿using System;
using System.Collections.Generic;
using System.Linq;

namespace LooCast.System
{
    using LooCast.System.Paths;

    public sealed class FolderManager : ModuleManager
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
        private FolderManager() : base("FolderManager", SystemManager.Instance)
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

        public bool TryGetFolder(FolderPath folderPath, out IFolder folder)
        {
            if (folderPath == "/")
            {
                folder = MainManager.Instance;
                return true;
            }
            if (!registeredFolders.ContainsKey(folderPath))
            {
                folder = null;
                return false;
            }
            else
            {
                folder = registeredFolders[folderPath];
                return true;
            }
        }

        public IFolder GetFolder(string folderGUSP)
        {
            if (!FolderPath.TryParse(folderGUSP, out FolderPath? folderPath))
            {
                return null;
            }
            return GetFolder(folderPath!);
        }

        public bool TryGetFolder(string stringFolderPath, out IFolder folder)
        {
            if (!FolderPath.TryParse(stringFolderPath, out FolderPath? folderPath))
            {
                folder = null;
                return false;
            }
            return TryGetFolder(folderPath!, out folder);
        }

        public bool FolderExists(FolderPath folderPath)
        {
            if (folderPath == "/")
            {
                return true;
            }
            return registeredFolders.ContainsKey(folderPath);
        }

        public IFolder CreateFolder(FolderPath folderPath)
        {
            if (folderPath == null)
            {
                throw new ArgumentNullException(nameof(folderPath));
            }

            if (FolderExists(folderPath))
            {
                return null;
            }

            FolderPath parentFolderPath = folderPath.ParentFolderPath;

            if (!TryGetFolder(parentFolderPath, out IFolder parentFolder))
            {
                CreateFolder(parentFolderPath);
                parentFolder = GetFolder(parentFolderPath);
            }

            IFolder folder = new Folder(folderPath.FolderName, parentFolder);
            RegisterFolder(folder);
            return folder;
        }

        public void DeleteFolder(IFolder folder, bool recursive = false)
        {
            if (folder == null)
            {
                throw new ArgumentNullException(nameof(folder));
            }

            if (!FolderExists(folder.FolderPath))
            {
                return;
            }

            if (recursive)
            {
                foreach (IFolder childFolder in ((IParent<IFolder>)folder).Children)
                {
                    DeleteFolder(childFolder, true);
                }

                foreach (IFile childFile in ((IParent<IFile>)folder).Children)
                {
                    FileManager.Instance.DeleteFile(childFile, true);
                }
            }
            else
            {
                if (((IParent<IFolder>)folder).Children.Count() != 0 || ((IParent<IFile>)folder).Children.Count() != 0)
                {
                    throw new InvalidOperationException("Folder is not empty!");
                }
                else
                {
                    UnregisterFolder(folder);
                }
            }
        }
        #endregion
    }
}