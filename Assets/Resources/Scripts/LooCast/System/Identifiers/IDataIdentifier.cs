using System;
using System.Collections.Generic;

namespace LooCast.System.Identifiers
{
    public interface IDataIdentifier : IIdentifier
    {
        #region Properties
        public string DataGUSID { get; }
        #endregion
    }
}
