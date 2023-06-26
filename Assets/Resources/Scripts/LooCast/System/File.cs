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

        #region Initialization Phase Flags
        public bool IsEarlyPreInitializing { get; protected set; }
        public bool IsPreInitializing { get; protected set; }
        public bool IsLatePreInitializing { get; protected set; }
        public bool IsEarlyPreInitialized { get; protected set; }
        public bool IsPreInitialized { get; protected set; }
        public bool IsLatePreInitialized { get; protected set; }

        public bool IsEarlyInitializing { get; protected set; }
        public bool IsInitializing { get; protected set; }
        public bool IsLateInitializing { get; protected set; }
        public bool IsEarlyInitialized { get; protected set; }
        public bool IsInitialized { get; protected set; }
        public bool IsLateInitialized { get; protected set; }

        public bool IsEarlyPostInitializing { get; protected set; }
        public bool IsPostInitializing { get; protected set; }
        public bool IsLatePostInitializing { get; protected set; }
        public bool IsEarlyPostInitialized { get; protected set; }
        public bool IsPostInitialized { get; protected set; }
        public bool IsLatePostInitialized { get; protected set; }

        public bool IsFullyPreInitialized
        {
            get
            {
                return IsEarlyPreInitialized && IsPreInitialized && IsLatePreInitialized;
            }
        }
        public bool IsFullyInitialized
        {
            get
            {
                return IsEarlyInitialized && IsInitialized && IsLateInitialized;
            }
        }
        public bool IsFullyPostInitialized
        {
            get
            {
                return IsEarlyPostInitialized && IsPostInitialized && IsLatePostInitialized;
            }
        }
        public bool IsCompletelyInitialized
        {
            get
            {
                return IsFullyPreInitialized && IsFullyInitialized && IsPostInitialized;
            }
        }
        #endregion

        #region Termination Phase Flags
        public bool IsEarlyPreTerminating { get; protected set; }
        public bool IsPreTerminating { get; protected set; }
        public bool IsLatePreTerminating { get; protected set; }
        public bool IsEarlyPreTerminated { get; protected set; }
        public bool IsPreTerminated { get; protected set; }
        public bool IsLatePreTerminated { get; protected set; }

        public bool IsEarlyTerminating { get; protected set; }
        public bool IsTerminating { get; protected set; }
        public bool IsLateTerminating { get; protected set; }
        public bool IsEarlyTerminated { get; protected set; }
        public bool IsTerminated { get; protected set; }
        public bool IsLateTerminated { get; protected set; }

        public bool IsEarlyPostTerminating { get; protected set; }
        public bool IsPostTerminating { get; protected set; }
        public bool IsLatePostTerminating { get; protected set; }
        public bool IsEarlyPostTerminated { get; protected set; }
        public bool IsPostTerminated { get; protected set; }
        public bool IsLatePostTerminated { get; protected set; }

        public bool IsFullyPreTerminated
        {
            get
            {
                return IsEarlyPreTerminated && IsPreTerminated && IsLatePreTerminated;
            }
        }
        public bool IsFullyTerminated
        {
            get
            {
                return IsEarlyTerminated && IsTerminated && IsLateTerminated;
            }
        }
        public bool IsFullyPostTerminated
        {
            get
            {
                return IsEarlyPostTerminated && IsPostTerminated && IsLatePostTerminated;
            }
        }
        public bool IsCompletelyTerminated
        {
            get
            {
                return IsFullyPreTerminated && IsFullyTerminated && IsPostTerminated;
            }
        }
        #endregion

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
