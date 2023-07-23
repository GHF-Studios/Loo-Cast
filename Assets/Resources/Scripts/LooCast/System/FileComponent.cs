using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.ECS;
    using LooCast.System.Paths;

    [IncompatibleComponents(typeof(FolderComponent), typeof(ObjectComponent))]
    public sealed class FileComponent : Component, IFileComponent
    {
        #region Classes
        new public class Data : Component.Data, IFileComponent.IData
        {
            #region Properties
            public string FileName { get; set; }
            public string FileExtension { get; set; }
            public FolderPath? ParentFolderPath { get; set; }
            #endregion
        }
        #endregion

        #region Properties
        public string FileName { get; private set; }
        public string FileExtension { get; private set; }
        public string FileIdentifier { get; private set; }

        public IHierarchicalElementPath HierarchicalElementPath => FilePath;
        public FilePath FilePath { get; private set; }

        public HierarchicalElementType HierarchyElementType => HierarchicalElementType.File;

        IFolderComponent IChild<IFolderComponent>.Parent => FolderParent;
        public IFolderComponent FolderParent { get; private set; }

        IEnumerable<IObjectComponent> IParent<IObjectComponent>.Children => ObjectChildren;
        public IEnumerable<IObjectComponent> ObjectChildren => objectChildrenList;
        #endregion

        #region Fields
        private List<IObjectComponent> objectChildrenList;
        #endregion

        #region Constructors
        public FileComponent() : base()
        {
            objectChildrenList = new List<IObjectComponent>();
            
            RegisterPreInitializationAction(() =>
            {
                FileManager.Instance.RegisterFile(this);
            });

            RegisterPostTerminationAction(() =>
            {
                FileManager.Instance.UnregisterFile(this);
                FileName = null;
                FileExtension = null;
                FileIdentifier = null;
                objectChildrenList = null;
            });
        }
        #endregion

        #region Methods
        public bool Validate()
        {
            return true;
        }

        #region Child Management
        public bool TryAddChildObject(IObjectComponent childObject)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"File '{this}' is not created yet!");
            }

            if (ContainsChildObject(childObject.ObjectName))
            {
                return false;
            }
            else
            {
                AddChildObject(childObject);
                return true;
            }
        }
        public void AddChildObject(IObjectComponent childObject)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"File '{this}' is not created yet!");
            }

            if (ContainsChildObject(childObject))
            {
                throw new InvalidOperationException($"File '{this}' already contains an Object '{childObject}'!");
            }
            objectChildrenList.Add(childObject);
        }

        public bool TryRemoveChildObject(IObjectComponent childObject)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"File '{this}' is not created yet!");
            }

            if (!ContainsChildObject(childObject))
            {
                return false;
            }
            else
            {
                RemoveChildObject(childObject);
                return true;
            }
        }
        public void RemoveChildObject(IObjectComponent childObject)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"File '{this}' is not created yet!");
            }

            objectChildrenList.Remove(childObject);
        }

        public bool TryGetChildObject(string childObjectName, out IObjectComponent childObject)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"File '{this}' is not created yet!");
            }

            if (!ContainsChildObject(childObjectName))
            {
                childObject = null;
                return false;
            }
            else
            {
                childObject = GetChildObject(childObjectName);
                return true;
            }
        }
        public IObjectComponent GetChildObject(string childObjectName)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"File '{this}' is not created yet!");
            }

            return objectChildrenList.Find((objectChild) => { return objectChild.ObjectName == childObjectName; });
        }

        public bool ContainsChildObject(string childObjectName)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"File '{this}' is not created yet!");
            }

            return objectChildrenList.Exists((objectChild) => { return objectChild.ObjectName == childObjectName; });
        }

        public bool ContainsChildObject(IObjectComponent childObject)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"File '{this}' is not created yet!");
            }

            return objectChildrenList.Contains(childObject);
        }

        public void ClearChildObjects()
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"File '{this}' is not created yet!");
            }

            objectChildrenList.Clear();
        }
        #endregion

        #region Data Management
        public override IComponent.IData GetComponentData()
        {
            IFileComponent.IData fileComponentData = (IFileComponent.IData)base.GetComponentData();

            fileComponentData.FileName = FileName;
            fileComponentData.FileExtension = FileExtension;
            fileComponentData.ParentFolderPath = FolderParent.FolderPath;

            return fileComponentData;
        }

        public override void SetComponentData(IComponent.IData componentData)
        {
            IFileComponent.IData fileComponentData = (IFileComponent.IData)componentData;
            
            PathBuilder filePathBuilder;
            
            if (((FolderPath)fileComponentData.ParentFolderPath).IsRoot)
            {
                filePathBuilder = PathBuilder.Create();
            }
            else
            {
                filePathBuilder = PathBuilder.Load((FolderPath)fileComponentData.ParentFolderPath);
            }
            
            filePathBuilder.AsAbsolutePath();
            filePathBuilder.WithFile(fileComponentData.FileName, fileComponentData.FileExtension);
            FilePath = filePathBuilder.ConstructFilePath();
            FileName = fileComponentData.FileName;
            FileExtension = fileComponentData.FileExtension;
            FileIdentifier = $"{fileComponentData.FileName}.{fileComponentData.FileExtension}";
            FolderParent = FolderManager.Instance.GetFolder(fileComponentData.ParentFolderPath);

            FolderParent.AddChildFile(this);

            base.SetComponentData(componentData);
        }
        #endregion

        #endregion

        #region Overrides
        public override string ToString()
        {
            return $"FileComponent[{FilePath}]";
        }

        public override bool Equals(object obj)
        {
            if (obj is FileComponent)
            {
                return Equals((FileComponent)obj);
            }
            else
            {
                return false;
            }
        }

        public bool Equals(FileComponent otherFile)
        {
            return otherFile.FilePath == this.FilePath;
        }

        public override int GetHashCode()
        {
            return FilePath.GetHashCode();
        }
        #endregion

        #region Operators
        public static bool operator ==(FileComponent file1, FileComponent file2)
        {
            if ((file1 is null && file2 is not null) || (file1 is not null && file2 is null))
            {
                return false;
            }
            else if (file1 is null && file2 is null)
            {
                return true;
            }
            else
            {
                return file1.Equals(file2);
            }
        }

        public static bool operator !=(FileComponent file1, FileComponent file2)
        {
            if ((file1 is null && file2 is not null) || (file1 is not null && file2 is null))
            {
                return true;
            }
            else if (file1 is null && file2 is null)
            {
                return false;
            }
            else
            {
                return !file1.Equals(file2);
            }
        }
        
        public static implicit operator string(FileComponent file)
        {
            return file.FilePath;
        }
        #endregion
    }
}
