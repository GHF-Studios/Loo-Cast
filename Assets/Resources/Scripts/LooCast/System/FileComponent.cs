using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using global::LooCast.System.ECS;
    using global::LooCast.System.Paths;

    [IncompatibleComponents(typeof(FolderComponent), typeof(ObjectComponent))]
    public sealed class FileComponent : Component, IFile
    {
        #region Properties
        public bool IsInitialized { get; private set; }

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
            IsInitialized = false;
        }
        #endregion

        #region Methods
        public void Initialize(string fileName, string fileExtension, IFolder folderParent)
        {
            if (IsInitialized)
            {
                throw new InvalidOperationException("File has already been initialized!");
            }

            PathBuilder filePathBuilder;

            if (folderParent == null)
            {
                filePathBuilder = PathBuilder.Create();
            }
            else
            {
                filePathBuilder = PathBuilder.Load(folderParent.FolderPath);
            }

            filePathBuilder.AsAbsolutePath();
            filePathBuilder.WithFile(fileName, fileExtension);

            FileName = fileName;
            FileExtension = fileExtension;
            FileIdentifier = $"{fileName}.{fileExtension}";
            FilePath = filePathBuilder.ConstructFilePath();
            FolderParent = folderParent;
            objectChildrenList = new List<IObject>();

            folderParent.AddChildFile(this);

            FileManager.Instance.RegisterFile(this);

            IsInitialized = true;
        }

        public bool Validate()
        {
            return true;
        }

        public bool TryAddChildObject(IObject childObject) 
        {
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
            if (ContainsChildObject(childObject))
            {
                throw new InvalidOperationException($"File '{this}' already contains an Object '{childObject}'!");
            }
            objectChildrenList.Add(childObject);
        }

        public bool TryRemoveChildObject(IObject childObject)
        {
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
            objectChildrenList.Remove(childObject);
        }

        public bool TryGetChildObject(string childObjectName, out IObject childObject) 
        {
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
            return objectChildrenList.Find((objectChild) => { return objectChild.ObjectName == childObjectName; });
        }

        public bool ContainsChildObject(string childObjectName) 
        {
            return objectChildrenList.Exists((objectChild) => { return objectChild.ObjectName == childObjectName; });
        }

        public bool ContainsChildObject(IObject childObject)
        {
            return objectChildrenList.Contains(childObject);
        }

        public void ClearChildObjects() 
        {
            objectChildrenList.Clear();
        }
        #endregion

        #region Overrides
        public override string ToString()
        {
            return FilePath;
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
