using System.Collections.Generic;

namespace LooCast.System.MetaData
{
    public interface IMetaData : IHierarchyElement
    {
        #region Properties
        public IIdentifier Identifier { get; }
        public IMetaData ParentMetaData { get; }
        public IEnumerable<IMetaData> ChildMetaData { get; }
        
        public ILooCastObject Parent { get; }
        public IEnumerable<ILooCastObject> Children { get; }
        #endregion

        #region Methods
        public bool Validate();
        public void PreInitialize();
        public void Initialize();
        public void PostInitialize();
        #endregion
    }
}
