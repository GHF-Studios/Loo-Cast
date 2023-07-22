using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.ECS;
    using LooCast.System.Paths;
    using LooCast.System.Serialization;

    [IncompatibleComponents(typeof(FileComponent), typeof(ObjectComponent))]
    public sealed class FolderComponent : Component, IFolder, ISerializable<Component.MetaData, FolderComponent.Data>
    {
        #region Classes
        new public class Data : Component.Data
        {
            #region Properties
            public string FolderName { get; set; }
            public FolderPath? ParentFolderPath { get; set; }
            #endregion

            #region Constructors
            public Data(string assemblyQualifiedComponentTypeName) : base(assemblyQualifiedComponentTypeName)
            {
                FolderName = "Root";
                ParentFolderPath = null;
            }

            public Data(string assemblyQualifiedComponentTypeName, string folderName, FolderPath parentFolderPath) : base(assemblyQualifiedComponentTypeName)
            {
                FolderName = folderName;
                ParentFolderPath = parentFolderPath;
            }
            #endregion
        }
        #endregion

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
        public FolderComponent() : base()
        {
            folderChildrenList = new List<IFolder>();
            fileChildrenList = new List<IFile>();
            
            RegisterPreInitializationAction(() =>
            {
            });

            RegisterPostTerminationAction(() =>
            {
                FolderName = null;
                folderChildrenList = null;
                fileChildrenList = null;
            });
        }
        #endregion

        #region Methods
        public bool Validate()
        {
            return true;
        }

        #region Child Management
        public bool TryAddChildFolder(IFolder childFolder) 
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Folder '{this}' is not created yet!");
            }
            
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
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Folder '{this}' is not created yet!");
            }

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
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Folder '{this}' is not created yet!");
            }

            if (ContainsChildFolder(childFolder))
            {
                throw new InvalidOperationException($"Folder '{this}' already contains a Folder '{childFolder}'!");
            }
            folderChildrenList.Add(childFolder);
        }
        public void AddChildFile(IFile childFile)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Folder '{this}' is not created yet!");
            }

            if (ContainsChildFile(childFile))
            {
                throw new InvalidOperationException($"Folder '{this}' already contains a File '{childFile}'!");
            }
            fileChildrenList.Add(childFile);
        }

        public bool TryRemoveChildFolder(IFolder childFolder)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Folder '{this}' is not created yet!");
            }

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
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Folder '{this}' is not created yet!");
            }

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
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Folder '{this}' is not created yet!");
            }

            folderChildrenList.Remove(childFolder);
        }
        public void RemoveChildFile(IFile childFile)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Folder '{this}' is not created yet!");
            }

            fileChildrenList.Remove(childFile);
        }

        public bool TryGetChildFolder(string childFolderName, out IFolder childFolder)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Folder '{this}' is not created yet!");
            }

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
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Folder '{this}' is not created yet!");
            }

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
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Folder '{this}' is not created yet!");
            }

            return folderChildrenList.Find((folderChild) => { return folderChild.FolderName == childFolderName; } );
        }
        public IFile GetChildFile(string childFileName, string childFileExtension)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Folder '{this}' is not created yet!");
            }

            return fileChildrenList.Find((fileChild) => { return fileChild.FileName == childFileName && fileChild.FileExtension == childFileExtension; });
        }
        public bool ContainsChildFolder(string childFolderName)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Folder '{this}' is not created yet!");
            }

            return folderChildrenList.Exists((childFolder) => { return childFolder.FolderName == childFolderName; });
        }
        public bool ContainsChildFile(string childFileName, string childFileExtension)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Folder '{this}' is not created yet!");
            }

            return fileChildrenList.Exists((fileChild) => { return fileChild.FileName == childFileName && fileChild.FileExtension == childFileExtension; });
        }
        public bool ContainsChildFolder(IFolder childFolder)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Folder '{this}' is not created yet!");
            }

            return folderChildrenList.Contains(childFolder);
        }
        public bool ContainsChildFile(IFile childFile)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Folder '{this}' is not created yet!");
            }

            return fileChildrenList.Contains(childFile);
        }

        public void ClearChildFolders()
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Folder '{this}' is not created yet!");
            }

            folderChildrenList.Clear();
        }
        public void ClearChildFiles()
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Folder '{this}' is not created yet!");
            }

            fileChildrenList.Clear();
        }
        #endregion

        #region Data Management
        Component.MetaData ISerializable<Component.MetaData, FolderComponent.Data>.GetMetaData()
        {
            return ((ISerializable<Component.MetaData, Component.Data>)this).GetMetaData();
        }

        FolderComponent.Data ISerializable<Component.MetaData, FolderComponent.Data>.GetData()
        {
            if (!HasData)
            {
                throw new InvalidOperationException($"FolderComponent '{this}' does not have data!");
            }
            
            return new FolderComponent.Data(ComponentType.AssemblyQualifiedName, FolderName, FolderParent.FolderPath);
        }

        void ISerializable<Component.MetaData, FolderComponent.Data>.SetMetaData(Component.MetaData metaData)
        {
            ((ISerializable<Component.MetaData, Component.Data>)this).SetMetaData(metaData);
        }

        void ISerializable<Component.MetaData, FolderComponent.Data>.SetData(FolderComponent.Data data)
        {
            if (data.FolderName.Equals("Root"))
            {
                IsRoot = true;
                FolderPath = new FolderPath(false);
                FolderName = "Root";
                FolderParent = null;
            }
            else
            {
                IsRoot = false;
                PathBuilder folderPathBuilder = PathBuilder.Load((FolderPath)data.ParentFolderPath);
                folderPathBuilder.AsAbsolutePath();
                folderPathBuilder.WithFolder(data.FolderName);
                FolderPath = folderPathBuilder.ConstructFolderPath();
                FolderName = data.FolderName;
                FolderParent.AddChildFolder(this);
            }
            
            ((ISerializable<Component.MetaData, Component.Data>)this).SetData(data);
        }
        #endregion

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
