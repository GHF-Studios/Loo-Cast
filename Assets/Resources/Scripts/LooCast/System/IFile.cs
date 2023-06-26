using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Paths;

    public interface IFile : IHierarchicalElement, IChild<IFolder>, IParent<IObject>
    {
        #region Properties
        string FileName { get; }
        string FileExtension { get; }
        FilePath FilePath { get; }
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
