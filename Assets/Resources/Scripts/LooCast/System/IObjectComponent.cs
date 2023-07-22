using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.ECS;
    using LooCast.System.Paths;

    public interface IObjectComponent : IComponent, IHierarchicalElement, IChild<IFileComponent>, IChild<IObjectComponent>, IParent<IObjectComponent>
    {
        #region Interfaces
        new public interface IData : IComponent.IData
        {
            #region Properties
            string ObjectName { get; set; }
            bool HasFileParent { get; set; }
            FilePath? ParentFilePath { get; set; }
            ObjectPath? ParentObjectPath { get; set; }
            #endregion
        }
        #endregion

        #region Properties
        string ObjectName { get; }
        ObjectPath ObjectPath{ get; }
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
