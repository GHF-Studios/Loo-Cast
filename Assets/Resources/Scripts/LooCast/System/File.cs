using System.Collections.Generic;

namespace LooCast.System
{
    using global::System;
    using LooCast.System.Paths;
    
    public class File : IFile
    {
        #region Properties
        public string FileName { get; private set; }
        public string FileExtension { get; private set; }

        public IHierarchicalElementPath HierarchicalElementPath => FilePath;
        public FilePath FilePath { get; private set; }

        public HierarchicalElementType HierarchyElementType => HierarchicalElementType.File;

        IFolder IChild<IFolder>.Parent => FolderParent;
        public IFolder FolderParent { get; private set; }

        IEnumerable<IObject> IParent<IObject>.Children => ObjectChildren;
        public IEnumerable<IObject> ObjectChildren => ObjectChildrenList;
        #endregion

        #region Fields
        protected List<IObject> ObjectChildrenList;
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
            
            filePathBuilder.WithFile(fileName, fileExtension);

            FileName = fileName;
            FileExtension = fileExtension;
            FilePath = filePathBuilder.ConstructFilePath();
            FolderParent = folderParent;
            ObjectChildrenList = new List<IObject>();
        }
        #endregion

        #region Methods
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
            ObjectChildrenList.Add(childObject);
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
            ObjectChildrenList.Remove(childObject);
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
            return ObjectChildrenList.Find((objectChild) => { return objectChild.ObjectName == childObjectName; });
        }

        public bool ContainsChildObject(string childObjectName) 
        {
            return ObjectChildrenList.Exists((objectChild) => { return objectChild.ObjectName == childObjectName; });
        }

        public bool ContainsChildObject(IObject childObject)
        {
            return ObjectChildrenList.Contains(childObject);
        }

        public void ClearChildObjects() 
        {
            ObjectChildrenList.Clear();
        }

        #region Initialization Phases
        public virtual void EarlyPreInitialize()
        {

        }

        public virtual void PreInitialize()
        {

        }

        public virtual void LatePreInitialize()
        {

        }

        public virtual void EarlyInitialize()
        {

        }

        public virtual void Initialize()
        {

        }

        public virtual void LateInitialize()
        {

        }

        public virtual void EarlyPostInitalize()
        {

        }

        public virtual void PostInitialize()
        {

        }

        public virtual void LatePostInitialize()
        {

        }
        #endregion

        #region Termination Phases
        public virtual void EarlyPreTerminate()
        {

        }

        public virtual void PreTerminate()
        {

        }

        public virtual void LatePreTerminate()
        {

        }

        public virtual void EarlyTerminate()
        {

        }

        public virtual void Terminate()
        {

        }

        public virtual void LateTerminate()
        {

        }

        public virtual void EarlyPostTerminate()
        {

        }

        public virtual void PostTerminate()
        {

        }

        public virtual void LatePostTerminate()
        {

        }
        #endregion

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
            return file1.Equals(file2);
        }

        public static bool operator !=(File file1, File file2)
        {
            return !file1.Equals(file2);
        }
        
        public static implicit operator string(File file)
        {
            return file.FilePath;
        }
        #endregion
    }
}
