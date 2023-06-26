using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Paths;
    
    public interface IFolder : IHierarchicalElement, IChild<IFolder>, IParent<IFolder>, IParent<IFile>
    {
        #region Properties
        string FolderName { get; }
        FolderPath FolderPath { get; }
        #endregion

        #region Methods
        public bool TryAddChildFolder(IFolder childFolder);
        public bool TryAddChildFile(IFile childFile);
        public void AddChildFolder(IFolder childFolder);
        public void AddChildFile(IFile childFile);

        public bool TryRemoveChildFolder(IFolder childFolder);
        public bool TryRemoveChildFile(IFile childFile);
        public void RemoveChildFolder(IFolder childFolder);
        public void RemoveChildFile(IFile childFile);

        public bool TryGetChildFolder(string childFolderName, out IFolder childFolder);
        public bool TryGetChildFile(string childFileName, string childFileExtension, out IFile childFile);
        public IFolder GetChildFolder(string childFolderName);
        public IFile GetChildFile(string childFileName, string childFileExtension);

        public bool ContainsChildFolder(string childFolderName);
        public bool ContainsChildFolder(IFolder childFolder);
        public bool ContainsChildFile(string childFileName, string childFileExtension);
        public bool ContainsChildFile(IFile childFile);

        public void ClearChildFolders();
        public void ClearChildFiles();
        #endregion
    }
}
