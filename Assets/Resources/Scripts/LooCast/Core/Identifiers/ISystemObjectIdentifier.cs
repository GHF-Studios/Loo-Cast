using System;
using System.Collections.Generic;

namespace LooCast.Core.Identifiers
{
    public interface ISystemObjectIdentifier : IInstanceIdentifier
    {
        #region Properties
        public TypeIdentifier SystemObjectTypeIdentifier { get; }
        public Guid SystemObjectInstanceGUID { get; }
        #endregion
    }
}
