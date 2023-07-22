using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.ECS;
    using LooCast.System.Paths;
    using LooCast.System.Serialization;

    [IncompatibleComponents(typeof(FileComponent), typeof(ObjectComponent))]
    public sealed class FolderComponent : Component, IFolderComponent
    {
        #region Classes
        new public class Data : Component.Data, IFolderComponent.IData
        {
            #region Properties
            public string FolderName { get; set; }
            public FolderPath? ParentFolderPath { get; set; }
            #endregion
        }
        #endregion

        #region Properties
        public string FolderName { get; private set; }
        public bool IsRoot { get; private set; }

        public IHierarchicalElementPath HierarchicalElementPath => FolderPath;
        public FolderPath FolderPath { get; private set; }

        public HierarchicalElementType HierarchyElementType => HierarchicalElementType.Folder;

        IFolderComponent IChild<IFolderComponent>.Parent => FolderParent;
        public IFolderComponent FolderParent { get; private set; }

        IEnumerable<IFolderComponent> IParent<IFolderComponent>.Children => FolderChildren;
        public IEnumerable<IFolderComponent> FolderChildren => folderChildrenList;

        IEnumerable<IFileComponent> IParent<IFileComponent>.Children => FileChildren;
        public IEnumerable<IFileComponent> FileChildren => fileChildrenList;
        #endregion

        #region Fields
        private List<IFolderComponent> folderChildrenList;
        private List<IFileComponent> fileChildrenList;
        #endregion

        #region Constructors
        public FolderComponent() : base()
        {
            folderChildrenList = new List<IFolderComponent>();
            fileChildrenList = new List<IFileComponent>();
            
            RegisterPreInitializationAction(() =>
            {
                FolderManager.Instance.RegisterFolder(this);
            });

            RegisterPostTerminationAction(() =>
            {
                FolderManager.Instance.UnregisterFolder(this);
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
        public bool TryAddChildFolder(IFolderComponent childFolder) 
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
        public bool TryAddChildFile(IFileComponent childFile)
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
        public void AddChildFolder(IFolderComponent childFolder)
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
        public void AddChildFile(IFileComponent childFile)
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

        public bool TryRemoveChildFolder(IFolderComponent childFolder)
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
        public bool TryRemoveChildFile(IFileComponent childFile)
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
        public void RemoveChildFolder(IFolderComponent childFolder)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Folder '{this}' is not created yet!");
            }

            folderChildrenList.Remove(childFolder);
        }
        public void RemoveChildFile(IFileComponent childFile)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Folder '{this}' is not created yet!");
            }

            fileChildrenList.Remove(childFile);
        }

        public bool TryGetChildFolder(string childFolderName, out IFolderComponent childFolder)
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
        public bool TryGetChildFile(string childFileName, string childFileExtension, out IFileComponent childFile)
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
        public IFolderComponent GetChildFolder(string childFolderName)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Folder '{this}' is not created yet!");
            }

            return folderChildrenList.Find((folderChild) => { return folderChild.FolderName == childFolderName; } );
        }
        public IFileComponent GetChildFile(string childFileName, string childFileExtension)
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
        public bool ContainsChildFolder(IFolderComponent childFolder)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Folder '{this}' is not created yet!");
            }

            return folderChildrenList.Contains(childFolder);
        }
        public bool ContainsChildFile(IFileComponent childFile)
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
        public override IData GetData()
        {
            IFolderComponent.IData folderComponentData = (IFolderComponent.IData)base.GetData();

            folderComponentData.FolderName = FolderName;
            folderComponentData.ParentFolderPath = FolderParent.FolderPath;

            return folderComponentData;
        }
        
        public override void SetData(IData data)
        {
            IFolderComponent.IData folderComponentData = (IFolderComponent.IData)data;
            
            if (folderComponentData.FolderName.Equals("Root"))
            {
                IsRoot = true;
                FolderPath = new FolderPath(false);
                FolderName = "Root";
                FolderParent = null;
            }
            else
            {
                IsRoot = false;
                PathBuilder folderPathBuilder = PathBuilder.Load((FolderPath)folderComponentData.ParentFolderPath);
                folderPathBuilder.AsAbsolutePath();
                folderPathBuilder.WithFolder(folderComponentData.FolderName);
                FolderPath = folderPathBuilder.ConstructFolderPath();
                FolderName = folderComponentData.FolderName;
                FolderParent = FolderManager.Instance.GetFolder(folderComponentData.ParentFolderPath);
                FolderParent.AddChildFolder(this);
            }

            base.SetData(data);
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
