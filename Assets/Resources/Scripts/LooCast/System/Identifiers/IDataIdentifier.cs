using System;
using System.Collections.Generic;

namespace LooCast.System.Identifiers
{
    public interface IDataIdentifier : IObjectIdentifier
    {
        #region Properties
        public string DataGUSID { get; }
        #endregion
    }
}
