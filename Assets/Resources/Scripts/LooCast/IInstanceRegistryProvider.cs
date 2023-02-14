namespace LooCast
{
    public interface IInstanceRegistryProvider
    {
        #region Properties
        Registry<InstanceIdentifier, Instance> LooCastInstances { get; }
        #endregion
    }
}