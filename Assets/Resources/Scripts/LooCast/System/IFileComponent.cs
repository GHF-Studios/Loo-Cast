using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.ECS;
    using LooCast.System.Paths;

    public interface IFileComponent : IComponent, IHierarchicalElement, IChild<IFolderComponent>, IParent<IObjectComponent>
    {
        #region Interfaces
        new public interface IData : IComponent.IData
        {
            #region Properties
            string FileName { get; set; }
            string FileExtension { get; set; }
            FolderPath? ParentFolderPath { get; set; }
            #endregion
        }
        #endregion

        #region Properties
        string FileName { get; }
        string FileExtension { get; }
        string FileIdentifier { get; }
        FilePath FilePath { get; }
        #endregion

        #region Methods
        bool TryAddChildObject(IObjectComponent childObject);
        void AddChildObject(IObjectComponent childObject);

        bool TryRemoveChildObject(IObjectComponent childObject);
        void RemoveChildObject(IObjectComponent childObject);

        bool TryGetChildObject(string childObjectName, out IObjectComponent childObject);
        IObjectComponent GetChildObject(string childObjectName);

        bool ContainsChildObject(string childObjectName);
        bool ContainsChildObject(IObjectComponent childObject);

        void ClearChildObjects();
        #endregion
    }
}
