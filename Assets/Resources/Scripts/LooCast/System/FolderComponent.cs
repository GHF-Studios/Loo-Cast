﻿using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using global::LooCast.System.ECS;
    using global::LooCast.System.Paths;

    [IncompatibleComponents(typeof(FileComponent), typeof(ObjectComponent))]
    public sealed class FolderComponent : Component, IFolder
    {
        #region Properties
        public bool IsSetup { get; private set; }

        public string FolderName { get; private set; }
        public bool IsRoot { get; private set; }

        public IHierarchicalElementPath HierarchicalElementPath => FolderPath;
        public FolderPath FolderPath { get; private set; }

        public HierarchicalElementType HierarchyElementType => HierarchicalElementType.Folder;

        IFolder IChild<IFolder>.Parent => FolderParent;
        public IFolder FolderParent { get; private set; }

        IEnumerable<IFolder> IParent<IFolder>.Children => FolderChildren;
        public IEnumerable<IFolder> FolderChildren => folderChildrenList;

        IEnumerable<IFile> IParent<IFile>.Children => FileChildren;
        public IEnumerable<IFile> FileChildren => fileChildrenList;
        #endregion

        #region Fields
        private List<IFolder> folderChildrenList;
        private List<IFile> fileChildrenList;
        #endregion

        #region Constructors
        public FolderComponent() : base()
        {
            IsSetup = false;
        }
        #endregion

        #region Methods
        public void SetupAsRoot()
        {
            if (IsSetup)
            {
                throw new InvalidOperationException("Folder has already been set up!");
            }

            IsRoot = true;

            FolderName = "Root";
            FolderPath = new FolderPath(false);
            FolderParent = null;
            folderChildrenList = new List<IFolder>();
            fileChildrenList = new List<IFile>();

            FolderManager.Instance.RegisterFolder(this);

            IsSetup = true;
        }

        public void Setup(string folderName, IFolder folderParent)
        {
            if (IsSetup)
            {
                throw new InvalidOperationException("Folder has already been set up!");
            }
            
            if (folderParent == null)
            {
                throw new NullReferenceException("FolderParent may not be null!");
            }

            IsRoot = false;
            PathBuilder folderPathBuilder = PathBuilder.Load(folderParent.FolderPath);

            folderPathBuilder.AsAbsolutePath();
            folderPathBuilder.WithFolder(folderName);

            FolderName = folderName;
            FolderPath = folderPathBuilder.ConstructFolderPath();
            FolderParent = folderParent;
            folderChildrenList = new List<IFolder>();
            fileChildrenList = new List<IFile>();

            folderParent.AddChildFolder(this);

            FolderManager.Instance.RegisterFolder(this);

            IsSetup = true;
        }

        public bool Validate()
        {
            return true;
        }

        public bool TryAddChildFolder(IFolder childFolder) 
        {
            if (ContainsChildFolder(childFolder.FolderName))
            {
                return false;
            }
            else
            {
                AddChildFolder(childFolder);
                return true;
            }
        }
        public bool TryAddChildFile(IFile childFile) 
        {
            if (ContainsChildFile(childFile))
            {
                return false;
            }
            else
            {
                AddChildFile(childFile);
                return true;
            }
        }
        public void AddChildFolder(IFolder childFolder) 
        {
            if (ContainsChildFolder(childFolder))
            {
                throw new InvalidOperationException($"Folder '{this}' already contains a Folder '{childFolder}'!");
            }
            folderChildrenList.Add(childFolder);
        }
        public void AddChildFile(IFile childFile) 
        {
            if (ContainsChildFile(childFile))
            {
                throw new InvalidOperationException($"Folder '{this}' already contains a File '{childFile}'!");
            }
            fileChildrenList.Add(childFile);
        }

        public bool TryRemoveChildFolder(IFolder childFolder) 
        {
            if (!ContainsChildFolder(childFolder))
            {
                return false;
            }
            else
            {
                RemoveChildFolder(childFolder);
                return true;
            }
        }
        public bool TryRemoveChildFile(IFile childFile) 
        {
            if (!ContainsChildFile(childFile))
            {
                return false;
            }
            else
            {
                RemoveChildFile(childFile);
                return true;
            }
        }
        public void RemoveChildFolder(IFolder childFolder) 
        {
            folderChildrenList.Remove(childFolder);
        }
        public void RemoveChildFile(IFile childFile) 
        {
            fileChildrenList.Remove(childFile);
        }

        public bool TryGetChildFolder(string childFolderName, out IFolder childFolder)
        {
            if (!ContainsChildFolder(childFolderName))
            {
                childFolder = null;
                return false;
            }
            else
            {
                childFolder = GetChildFolder(childFolderName);
                return true;
            }
        }
        public bool TryGetChildFile(string childFileName, string childFileExtension, out IFile childFile) 
        {
            if (!ContainsChildFile(childFileName, childFileExtension))
            {
                childFile = null;
                return false;
            }
            else
            {
                childFile = GetChildFile(childFileName, childFileExtension);
                return true;
            }
        }
        public IFolder GetChildFolder(string childFolderName) 
        {
            return folderChildrenList.Find((folderChild) => { return folderChild.FolderName == childFolderName; } );
        }
        public IFile GetChildFile(string childFileName, string childFileExtension)
        {
            return fileChildrenList.Find((fileChild) => { return fileChild.FileName == childFileName && fileChild.FileExtension == childFileExtension; });
        }
        public bool ContainsChildFolder(string childFolderName) 
        {
            return folderChildrenList.Exists((childFolder) => { return childFolder.FolderName == childFolderName; });
        }
        public bool ContainsChildFile(string childFileName, string childFileExtension)
        {
            return fileChildrenList.Exists((fileChild) => { return fileChild.FileName == childFileName && fileChild.FileExtension == childFileExtension; });
        }
        public bool ContainsChildFolder(IFolder childFolder)
        {
            return folderChildrenList.Contains(childFolder);
        }
        public bool ContainsChildFile(IFile childFile)
        {
            return fileChildrenList.Contains(childFile);
        }

        public void ClearChildFolders() 
        {
            folderChildrenList.Clear();
        }
        public void ClearChildFiles() 
        {
            fileChildrenList.Clear();
        }
        #endregion

        #region Overrides
        public override string ToString()
        {
            return $"FolderComponent[{FolderPath}]";
        }

        public override bool Equals(object obj)
        {
            if (obj is FolderComponent)
            {
                return Equals((FolderComponent)obj);
            }
            else
            {
                return false;
            }
        }

        public bool Equals(FolderComponent otherFolder)
        {
            return otherFolder.FolderPath == this.FolderPath;
        }

        public override int GetHashCode()
        {
            return FolderPath.GetHashCode();
        }
        #endregion

        #region Operators
        public static bool operator ==(FolderComponent folder1, FolderComponent folder2)
        {
            if ((folder1 is null && folder2 is not null) || (folder1 is not null && folder2 is null))
            {
                return false;
            }
            else if (folder1 is null && folder2 is null)
            {
                return true;
            }
            else
            {
                return folder1.Equals(folder2);
            }
        }

        public static bool operator !=(FolderComponent folder1, FolderComponent folder2)
        {
            if ((folder1 is null && folder2 is not null) || (folder1 is not null && folder2 is null))
            {
                return true;
            }
            else if (folder1 is null && folder2 is null)
            {
                return false;
            }
            else
            {
                return !folder1.Equals(folder2);
            }
        }

        public static implicit operator string(FolderComponent folder)
        {
            return folder.FolderPath;
        }
        #endregion
    }
}
