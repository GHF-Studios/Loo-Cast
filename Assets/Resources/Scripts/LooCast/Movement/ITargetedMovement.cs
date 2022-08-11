namespace LooCast.Movement
{
    using LooCast.Target;
    
    public interface ITargetedMovement : IMovement
    {
        Target Target { set; }
    } 
}
