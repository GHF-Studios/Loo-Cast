using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.ECS;
    using LooCast.System.Paths;
    
    public interface IFolderComponent : IComponent, IHierarchicalElement, IChild<IFolderComponent>, IParent<IFolderComponent>, IParent<IFileComponent>
    {
        #region Interfaces
        new public interface IData : IComponent.IData
        {
            #region Properties
            string FolderName { get; set; }
            FolderPath? ParentFolderPath { get; set; }
            #endregion
        }
        #endregion

        #region Properties
        string FolderName { get; }
        FolderPath FolderPath { get; }
        #endregion

        #region Methods
        bool TryAddChildFolder(IFolderComponent childFolder);
        bool TryAddChildFile(IFileComponent childFile);
        void AddChildFolder(IFolderComponent childFolder);
        void AddChildFile(IFileComponent childFile);

        bool TryRemoveChildFolder(IFolderComponent childFolder);
        bool TryRemoveChildFile(IFileComponent childFile);
        void RemoveChildFolder(IFolderComponent childFolder);
        void RemoveChildFile(IFileComponent childFile);

        bool TryGetChildFolder(string childFolderName, out IFolderComponent childFolder);
        bool TryGetChildFile(string childFileName, string childFileExtension, out IFileComponent childFile);
        IFolderComponent GetChildFolder(string childFolderName);
        IFileComponent GetChildFile(string childFileName, string childFileExtension);

        bool ContainsChildFolder(string childFolderName);
        bool ContainsChildFile(string childFileName, string childFileExtension);
        bool ContainsChildFolder(IFolderComponent childFolder);
        bool ContainsChildFile(IFileComponent childFile);

        void ClearChildFolders();
        void ClearChildFiles();
        #endregion
    }
}
