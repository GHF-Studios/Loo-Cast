namespace LooCast.System.Identification
{
    public interface IComponentIdentifier : IGameObjectIdentifier
    {
        #region Properties
        string ComponentTypeID { get; }
        string ComponentGUID { get; }
        string ComponentID { get; }
        #endregion
    }
}