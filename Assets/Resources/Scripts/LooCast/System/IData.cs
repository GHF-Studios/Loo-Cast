using System.Collections.Generic;

namespace LooCast.System
{
    public interface IData : IIdentifiable, IHierarchyElement
    {
        #region Properties
        IInstance Instance { get; }
        IData DataParent { get; }
        IEnumerable<IData> DataChildren { get; }
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
