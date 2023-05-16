using System.Collections.Generic;

namespace LooCast.System
{
    public interface IMetaData : IIdentifiable, IHierarchyElement
    {
        #region Properties
        IMetaData MetaDataParent { get; set; }
        IEnumerable<IMetaData> MetaDataChildren { get; set; }

        ILooCastObject Parent { get; set; }
        IEnumerable<ILooCastObject> Children { get; set; }
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
