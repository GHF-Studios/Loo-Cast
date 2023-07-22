using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.ECS;
    using LooCast.System.Paths;
    using LooCast.System.Serialization;

    [IncompatibleComponents(typeof(FolderComponent), typeof(ObjectComponent))]
    public sealed class FileComponent : Component, IFile, ISerializable<Component.MetaData, FileComponent.Data>
    {
        #region Classes
        new public class Data : Component.Data
        {
            #region Properties
            public string FileName { get; set; }
            public string FileExtension { get; set; }
            public FolderPath? ParentFolderPath { get; set; }
            #endregion

            #region Constructors
            public Data(string assemblyQualifiedComponentTypeName, string fileName, string fileExtension, FolderPath? parentFolderPath) : base(assemblyQualifiedComponentTypeName)
            {
                FileName = fileName;
                FileExtension = fileExtension;
                ParentFolderPath = parentFolderPath;
            }
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

        IFolder IChild<IFolder>.Parent => FolderParent;
        public IFolder FolderParent { get; private set; }

        IEnumerable<IObject> IParent<IObject>.Children => ObjectChildren;
        public IEnumerable<IObject> ObjectChildren => objectChildrenList;
        #endregion

        #region Fields
        private List<IObject> objectChildrenList;
        #endregion

        #region Constructors
        public FileComponent() : base()
        {
            objectChildrenList = new List<IObject>();
            
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
        public void Setup(string fileName, string fileExtension, IFolder folderParent)
        {
            
        }

        public bool Validate()
        {
            return true;
        }

        #region Child Management
        public bool TryAddChildObject(IObject childObject)
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
        public void AddChildObject(IObject childObject)
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

        public bool TryRemoveChildObject(IObject childObject)
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
        public void RemoveChildObject(IObject childObject)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"File '{this}' is not created yet!");
            }

            objectChildrenList.Remove(childObject);
        }

        public bool TryGetChildObject(string childObjectName, out IObject childObject)
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
        public IObject GetChildObject(string childObjectName)
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

        public bool ContainsChildObject(IObject childObject)
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
        Component.MetaData ISerializable<Component.MetaData, FileComponent.Data>.GetMetaData()
        {
            return ((ISerializable<Component.MetaData, Component.Data>)this).GetMetaData();
        }

        FileComponent.Data ISerializable<Component.MetaData, FileComponent.Data>.GetData()
        {
            if (!HasData)
            {
                throw new InvalidOperationException($"FileComponent '{this}' does not have data!");
            }

            return new FileComponent.Data(ComponentType.AssemblyQualifiedName, FileName, FileExtension, FolderParent.FolderPath);
        }

        void ISerializable<Component.MetaData, FileComponent.Data>.SetMetaData(Component.MetaData metaData)
        {
            ((ISerializable<Component.MetaData, Component.Data>)this).SetMetaData(metaData);
        }

        void ISerializable<Component.MetaData, FileComponent.Data>.SetData(FileComponent.Data data)
        {
            PathBuilder filePathBuilder;

            if (((FolderPath)data.ParentFolderPath).IsRoot)
            {
                filePathBuilder = PathBuilder.Create();
            }
            else
            {
                filePathBuilder = PathBuilder.Load((FolderPath)data.ParentFolderPath);
            }

            filePathBuilder.AsAbsolutePath();
            filePathBuilder.WithFile(data.FileName, data.FileExtension);

            FileName = data.FileName;
            FileExtension = data.FileExtension;
            FileIdentifier = $"{data.FileName}.{data.FileExtension}";
            FilePath = filePathBuilder.ConstructFilePath();
            FolderParent = FolderManager.Instance.GetFolder(data.ParentFolderPath);

            FolderParent.AddChildFile(this);

            ((ISerializable<Component.MetaData, Component.Data>)this).SetData(data);
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
