using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using global::LooCast.System.Paths;

    public interface IObject : IHierarchicalElement, IChild<IFile>, IChild<IObject>, IParent<IObject>
    {
        #region Properties
        string ObjectName { get; }
        ObjectPath ObjectPath{ get; }
        #endregion

        #region Methods
        bool TryAddChildObject(IObject childObject);
        void AddChildObject(IObject childObject);

        bool TryRemoveChildObject(IObject childObject);
        void RemoveChildObject(IObject childObject);

        bool TryGetChildObject(string childObjectName, out IObject childObject);
        IObject GetChildObject(string childObjectName);

        bool ContainsChildObject(string childObjectName);
        bool ContainsChildObject(IObject childObject);

        void ClearChildObjects();
        #endregion
    }
}
