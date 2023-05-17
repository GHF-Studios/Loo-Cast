using System.Collections.Generic;

namespace LooCast.System
{
    public interface IData
    {
        #region Properties
        IMetaData ContainingMetaData { get; }
        
        IData DataParent { get; }
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
