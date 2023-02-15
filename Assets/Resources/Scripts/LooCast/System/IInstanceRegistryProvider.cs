namespace LooCast.System
{
    using System.Identification;
    
    public interface IInstanceRegistryProvider
    {
        #region Properties
        Registry<InstanceIdentifier, Instance> LooCastInstances { get; }
        #endregion
    }
}