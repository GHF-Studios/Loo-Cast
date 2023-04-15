﻿namespace LooCast.System.Registries
{
    using global::LooCast.System.Identifiers;

    public sealed class GameObjectRegistry : Registry<GameObjectIdentifier, GameObject>
    {
        #region Overides
        protected override IRegistry GetBaseRegistry()
        {
            return MainManager.Instance.MainRegistry;
        }
        #endregion
    }
}
