using System;
using System.Collections.Generic;

namespace LooCast.System.Identifiers
{
    public interface IMetaDataIdentifier : IObjectIdentifier
    {
        #region Properties
        public string MetaDataGUSID { get; }
        #endregion
    }
}
