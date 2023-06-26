using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Paths;

    public interface IObject : IHierarchicalElement, IChild<IFile>, IChild<IObject>, IParent<IObject>
    {
        #region Properties
        string ObjectName { get; }
        ObjectPath ObjectPath{ get; }
        #endregion

        #region Methods
        public bool TryAddChildObject(IObject childObject);
        public void AddChildObject(IObject childObject);

        public bool TryRemoveChildObject(IObject childObject);
        public void RemoveChildObject(IObject childObject);

        public bool TryGetChildObject(string childObjectName, out IObject childObject);
        public IObject GetChildObject(string childObjectName);

        public bool ContainsChildObject(string childObjectName);
        public bool ContainsChildObject(IObject childObject);

        public void ClearChildObjects();
        #endregion
    }
}
