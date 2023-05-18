namespace LooCast.System.Registries
{
    using LooCast.System.Identifiers;

    public class InstanceRegistry<KeyType, ValueType> : Registry<KeyType, ValueType>
        where KeyType : IObjectIdentifier
        where ValueType : IIdentifiableObject
    {
        #region Overides
        protected override IRegistry GetBaseRegistry()
        {
            return MainManager.Instance.MainRegistry.GetRegistry(typeof(IInstance));
        }
        #endregion
    }
}
