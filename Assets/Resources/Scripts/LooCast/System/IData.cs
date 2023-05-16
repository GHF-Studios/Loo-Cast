using System.Collections.Generic;

namespace LooCast.System
{
    public interface IData : IIdentifiable, IHierarchyElement
    {
        #region Properties
        IData DataParent { get; set; }
        IEnumerable<IData> DataChildren { get; set; }
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
