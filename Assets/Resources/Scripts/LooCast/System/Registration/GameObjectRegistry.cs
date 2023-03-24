using System;

namespace LooCast.System.Registration
{
    using LooCast.System.Identification;
    using LooCast.System.Types;

    public class GameObjectRegistry : Registry<IGameObjectIdentifier, IGameObjectIdentifiable>, IGameObjectRegistry<IGameObjectIdentifier, IGameObjectIdentifiable>
    {
        public GameObjectRegistry(IType keyType, IType valueType) : base(keyType, valueType)
        {
            
        }
    }
}
