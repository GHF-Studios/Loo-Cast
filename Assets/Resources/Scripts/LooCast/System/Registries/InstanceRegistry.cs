namespace LooCast.System.Registries
{
    using global::LooCast.System.Identifiers;

    public class InstanceRegistry<KeyType, ValueType> : Registry<KeyType, ValueType>
        where KeyType : Identifier
        where ValueType : IType.IInstance
    {
        #region Overides
        protected override IRegistry GetBaseRegistry()
        {
            return MainManager.Instance.MainRegistry.GetRegistry(typeof(IType.IInstance));
        }
        #endregion
    }
}
