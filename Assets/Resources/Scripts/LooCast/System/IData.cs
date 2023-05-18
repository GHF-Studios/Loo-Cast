using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identifiers;

    public interface IData : IEngineObject
    {
        #region Properties
        IDataIdentifier DataIdentifier { get; }

        IData DataParent { get; }

        IMetaData ObjectMetaData { get; }
        #endregion
    }
}
