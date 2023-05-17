using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identifiers;

    public interface IData : ILooCastObject
    {
        #region Properties
        IDataIdentifier DataIdentifier { get; }

        IMetaData ObjectMetaData { get; }

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
