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
        bool TryAddChildFolder(IFolder childFolder);
        bool TryAddChildFile(IFile childFile);
        void AddChildFolder(IFolder childFolder);
        void AddChildFile(IFile childFile);

        bool TryRemoveChildFolder(IFolder childFolder);
        bool TryRemoveChildFile(IFile childFile);
        void RemoveChildFolder(IFolder childFolder);
        void RemoveChildFile(IFile childFile);

        bool TryGetChildFolder(string childFolderName, out IFolder childFolder);
        bool TryGetChildFile(string childFileName, string childFileExtension, out IFile childFile);
        IFolder GetChildFolder(string childFolderName);
        IFile GetChildFile(string childFileName, string childFileExtension);

        bool ContainsChildFolder(string childFolderName);
        bool ContainsChildFile(string childFileName, string childFileExtension);
        bool ContainsChildFolder(IFolder childFolder);
        bool ContainsChildFile(IFile childFile);

        void ClearChildFolders();
        void ClearChildFiles();
        #endregion
    }
}
