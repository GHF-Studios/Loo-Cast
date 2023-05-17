using System;
using System.Collections.Generic;

namespace LooCast.Core.Identifiers
{
    public interface IComponentIdentifier : IInstanceIdentifier
    {
        #region Properties
        public TypeIdentifier ComponentTypeIdentifier { get; }
        public Guid ComponentInstanceGUID { get; }
        public GameObjectIdentifier ContainingGameObjectIdentifier { get; }
        #endregion
    }
}
