using System.Collections.Generic;

namespace LooCast.System
{
    using global::System;
    using LooCast.System.Paths;

    public class Folder : IFolder
    {
        #region Properties
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
        public Folder()
        {
            IsRoot = true;
            
            FolderName = "Root";
            FolderPath = new FolderPath(false);
            FolderParent = null;
            folderChildrenList = new List<IFolder>();
            fileChildrenList = new List<IFile>();
        }

        public Folder(string folderName, IFolder folderParent)
        {
            IsRoot = false;

            PathBuilder folderPathBuilder = PathBuilder.Load(folderParent.FolderPath);
            folderPathBuilder.WithFolder(folderName);

            FolderName = folderName;
            FolderPath = folderPathBuilder.ConstructFolderPath();
            FolderParent = folderParent;
            folderChildrenList = new List<IFolder>();
            fileChildrenList = new List<IFile>();
        }
        #endregion

        #region Methods
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
            folderChildrenList.Add(childFolder);
        }
        public void AddChildFile(IFile childFile) 
        {
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
            return FolderPath;
        }

        public override bool Equals(object obj)
        {
            if (obj is Folder)
            {
                return Equals((Folder)obj);
            }
            else
            {
                return false;
            }
        }

        public bool Equals(Folder otherFolder)
        {
            return otherFolder.FolderPath == this.FolderPath;
        }

        public override int GetHashCode()
        {
            return FolderPath.GetHashCode();
        }
        #endregion

        #region Operators
        public static bool operator ==(Folder folder1, Folder folder2)
        {
            return folder1.Equals(folder2);
        }

        public static bool operator !=(Folder folder1, Folder folder2)
        {
            return !folder1.Equals(folder2);
        }

        public static implicit operator string(Folder folder)
        {
            return folder.FolderPath;
        }
        #endregion
    }
}
