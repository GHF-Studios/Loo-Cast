using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Paths;

    public class File : IFile
    {
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
        protected List<IObject> objectChildrenList;
        #endregion

        #region Constructors
        public File(string fileName, string fileExtension, IFolder folderParent)
        {
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
            FileIdentifier = fileName + fileExtension;
            FilePath = filePathBuilder.ConstructFilePath();
            FolderParent = folderParent;
            objectChildrenList = new List<IObject>();
        }
        #endregion

        #region Methods
        public virtual bool Validate()
        {
            return true;
        }

        public virtual bool TryAddChildObject(IObject childObject) 
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
        public virtual void AddChildObject(IObject childObject) 
        {
            objectChildrenList.Add(childObject);
        }

        public virtual bool TryRemoveChildObject(IObject childObject)
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
        public virtual void RemoveChildObject(IObject childObject) 
        {
            objectChildrenList.Remove(childObject);
        }

        public virtual bool TryGetChildObject(string childObjectName, out IObject childObject) 
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
        public virtual IObject GetChildObject(string childObjectName) 
        {
            return objectChildrenList.Find((objectChild) => { return objectChild.ObjectName == childObjectName; });
        }

        public virtual bool ContainsChildObject(string childObjectName) 
        {
            return objectChildrenList.Exists((objectChild) => { return objectChild.ObjectName == childObjectName; });
        }

        public virtual bool ContainsChildObject(IObject childObject)
        {
            return objectChildrenList.Contains(childObject);
        }

        public virtual void ClearChildObjects() 
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
            if (obj is File)
            {
                return Equals((File)obj);
            }
            else
            {
                return false;
            }
        }

        public bool Equals(File otherFile)
        {
            return otherFile.FilePath == this.FilePath;
        }

        public override int GetHashCode()
        {
            return FilePath.GetHashCode();
        }
        #endregion

        #region Operators
        public static bool operator ==(File file1, File file2)
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

        public static bool operator !=(File file1, File file2)
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
        
        public static implicit operator string(File file)
        {
            return file.FilePath;
        }
        #endregion
    }
}
