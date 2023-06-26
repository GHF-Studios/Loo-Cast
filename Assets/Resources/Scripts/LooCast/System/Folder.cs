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
        public IEnumerable<IFolder> FolderChildren => FolderChildrenList;

        IEnumerable<IFile> IParent<IFile>.Children => FileChildren;
        public IEnumerable<IFile> FileChildren => FileChildrenList;

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
        protected List<IFolder> FolderChildrenList;
        protected List<IFile> FileChildrenList;
        #endregion

        #region Constructors
        public Folder()
        {
            IsRoot = true;
            
            FolderName = "Root";
            FolderPath = new FolderPath(false);
            FolderParent = null;
            FolderChildrenList = new List<IFolder>();
            FileChildrenList = new List<IFile>();
        }

        public Folder(string folderName, IFolder folderParent)
        {
            IsRoot = false;

            PathBuilder folderPathBuilder = PathBuilder.Load(folderParent.FolderPath);
            folderPathBuilder.WithFolder(folderName);

            FolderName = folderName;
            FolderPath = folderPathBuilder.ConstructFolderPath();
            FolderParent = folderParent;
            FolderChildrenList = new List<IFolder>();
            FileChildrenList = new List<IFile>();
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
            FolderChildrenList.Add(childFolder);
        }
        public void AddChildFile(IFile childFile) 
        {
            FileChildrenList.Add(childFile);
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
            FolderChildrenList.Remove(childFolder);
        }
        public void RemoveChildFile(IFile childFile) 
        {
            FileChildrenList.Remove(childFile);
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
            return FolderChildrenList.Find((folderChild) => { return folderChild.FolderName == childFolderName; } );
        }
        public IFile GetChildFile(string childFileName, string childFileExtension)
        {
            return FileChildrenList.Find((fileChild) => { return fileChild.FileName == childFileName && fileChild.FileExtension == childFileExtension; });
        }
        public bool ContainsChildFolder(string childFolderName) 
        {
            return FolderChildrenList.Exists((childFolder) => { return childFolder.FolderName == childFolderName; });
        }
        public bool ContainsChildFolder(IFolder childFolder)
        {
            return FolderChildrenList.Contains(childFolder);
        }
        public bool ContainsChildFile(string childFileName, string childFileExtension) 
        {
            return FileChildrenList.Exists((fileChild) => { return fileChild.FileName == childFileName && fileChild.FileExtension == childFileExtension; });
        }
        public bool ContainsChildFile(IFile childFile)
        {
            return FileChildrenList.Contains(childFile);
        }

        public void ClearChildFolders() 
        {
            FolderChildrenList.Clear();
        }
        public void ClearChildFiles() 
        {
            FileChildrenList.Clear();
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
