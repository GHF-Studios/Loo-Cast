using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identifiers;
    
    public interface IMetaData : ILooCastObject
    {
        #region Properties
        IMetaDataIdentifier MetaDataIdentifier { get; }

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
