// Fill out your copyright notice in the Description page of Project Settings.

#include "CaptainFunctionalActor.h"


// Sets default values
ACaptainFunctionalActor::ACaptainFunctionalActor()
{
	// Set this character to call Tick() every frame.  You can turn this off to improve performance if you don't need it.
	PrimaryActorTick.bCanEverTick = true;

	Collider = CreateDefaultSubobject<UBoxComponent>(TEXT("Collider"));
	//Collider->Extend
	Collider->SetGenerateOverlapEvents(true);
	Collider->SetCollisionEnabled(ECollisionEnabled::QueryOnly);
}

// Called when the game starts or when spawned
void ACaptainFunctionalActor::BeginPlay()
{
	Super::BeginPlay();

	auto viewportSize = FVector2D(GEngine->GameViewport->Viewport->GetSizeXY());
	auto viewportCenter = FVector2D(viewportSize.X / 2, viewportSize.Y / 2);

	vector_min = FVector(0.0f, -(viewportCenter.X / 2.0f) + min_x_buffer, -(viewportCenter.Y / 2.0f) + min_y_buffer);
	vector_max = FVector(0.0f, (viewportCenter.X / 2.0f) - max_x_buffer, (viewportCenter.Y / 2.0f) - max_y_buffer);
}

// Called every frame
void ACaptainFunctionalActor::Tick(float DeltaTime)
{
	Super::Tick(DeltaTime);

	auto loc = GetActorLocation();

	if (move_up_pressed) {
		loc.Z += speed * DeltaTime;
	}

	if (move_down_pressed) {
		loc.Z -= speed * DeltaTime;
	}

	if (move_right_pressed) {
		loc.Y += speed * DeltaTime;
	}

	if (move_left_pressed) {
		loc.Y -= speed * DeltaTime;
	}

	loc = ClampVector(loc, vector_min, vector_max);

	SetActorLocation(loc);

}

// Called to bind functionality to input
void ACaptainFunctionalActor::SetupPlayerInputComponent(UInputComponent* PlayerInputComponent)
{
	Super::SetupPlayerInputComponent(PlayerInputComponent);

	FInputActionBinding move_up_pressed_binding{ "MoveUp", IE_Pressed };
	move_up_pressed_binding.ActionDelegate.GetDelegateForManualSet().BindLambda([this]() {move_up_pressed = true; });
	PlayerInputComponent->AddActionBinding(move_up_pressed_binding);

	FInputActionBinding move_up_released_binding{ "MoveUp", IE_Released };
	move_up_released_binding.ActionDelegate.GetDelegateForManualSet().BindLambda([this]() {move_up_pressed = false; });
	PlayerInputComponent->AddActionBinding(move_up_released_binding);

	FInputActionBinding move_down_pressed_binding{ "MoveDown", IE_Pressed };
	move_down_pressed_binding.ActionDelegate.GetDelegateForManualSet().BindLambda([this]() {move_down_pressed = true; });
	PlayerInputComponent->AddActionBinding(move_down_pressed_binding);

	FInputActionBinding move_down_released_binding{ "MoveDown", IE_Released };
	move_down_released_binding.ActionDelegate.GetDelegateForManualSet().BindLambda([this]() {move_down_pressed = false; });
	PlayerInputComponent->AddActionBinding(move_down_released_binding);

	FInputActionBinding move_right_pressed_binding{ "MoveRight", IE_Pressed };
	move_right_pressed_binding.ActionDelegate.GetDelegateForManualSet().BindLambda([this]() {move_right_pressed = true; });
	PlayerInputComponent->AddActionBinding(move_right_pressed_binding);

	FInputActionBinding move_right_released_binding{ "MoveRight", IE_Released };
	move_right_released_binding.ActionDelegate.GetDelegateForManualSet().BindLambda([this]() {move_right_pressed = false; });
	PlayerInputComponent->AddActionBinding(move_right_released_binding);

	FInputActionBinding move_left_pressed_binding{ "MoveLeft", IE_Pressed };
	move_left_pressed_binding.ActionDelegate.GetDelegateForManualSet().BindLambda([this]() {move_left_pressed = true; });
	PlayerInputComponent->AddActionBinding(move_left_pressed_binding);

	FInputActionBinding move_left_released_binding{ "MoveLeft", IE_Released };
	move_left_released_binding.ActionDelegate.GetDelegateForManualSet().BindLambda([this]() {move_left_pressed = false; });
	PlayerInputComponent->AddActionBinding(move_left_released_binding);
}

void ACaptainFunctionalActor::NotifyActorBeginOverlap(AActor* Other) {

	GEngine->AddOnScreenDebugMessage(-1, 5.f, FColor::Red, Other->GetName());
	SetHealth(GetHealth() - 1);

	Other->Destroy();

	//GEngine->AddOnScreenDebugMessage(-1, 5.f, FColor::Red, TEXT("Collision!"));
}

//UFUNCTION() void ACaptainFunctionalActor::OnOverlapBegin(UPrimitiveComponent * OverlappedComp, AActor * OtherActor, UPrimitiveComponent * OtherComp, int32 OtherBodyIndex, bool bFromSweep, const FHitResult & SweepResult)
//{
//	GEngine->AddOnScreenDebugMessage(-1, 5.f, FColor::Red, TEXT("Collision!"));
//}

int ACaptainFunctionalActor::GetHealth()
{
	return health;
}

void ACaptainFunctionalActor::SetHealth(int hp)
{
	health = hp;
	if (health <= 0) {
		GEngine->AddOnScreenDebugMessage(-1, 5.f, FColor::Red, TEXT("DEAD"));
	}
}
