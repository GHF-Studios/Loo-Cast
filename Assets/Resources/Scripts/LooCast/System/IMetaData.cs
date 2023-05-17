using System.Collections.Generic;

namespace LooCast.System
{
    public interface IMetaData
    {
        #region Properties
        IIdentifier ObjectIdentifier { get; }
        HierarchyElement ObjectHierarchyElement { get; }

        IMetaData MetaDataParent { get; }
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
