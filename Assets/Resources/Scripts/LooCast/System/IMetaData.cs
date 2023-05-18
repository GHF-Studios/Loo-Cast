using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identifiers;
    
    public interface IMetaData : IEngineObject
    {
        #region Properties
        IMetaDataIdentifier MetaDataIdentifier { get; }

        IMetaData MetaDataParent { get; }
        #endregion
    }
}
