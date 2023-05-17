using System.Collections.Generic;

namespace LooCast.System
{
    public interface IMetaData : IIdentifiable, IHierarchyElement
    {
        #region Properties
        IInstance Instance { get; }
        IMetaData MetaDataParent { get; }
        IEnumerable<IMetaData> MetaDataChildren { get; }
        #endregion

        #region Methods
        bool Validate();
        void PreInitialize();
        void Initialize();
        void PostInitialize();
        void PreTerminate();
        void Terminate();
        void PostTerminate();
        #endregion
    }
}
